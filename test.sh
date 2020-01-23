#!/bin/sh
set -e

CMD=$(target/debug/encrypt "Hello World!" | grep "Decrypt with:" | cut -d ' ' -f 3-99)
OK=$(eval $CMD | grep "Message:")
if [ "$OK" != "Message: Hello World!" ]; then
    echo "Encrypt/Decrypt failed :("
    exit -1
fi

CMD=$(target/debug/pkencrypt "Hello World!" | grep "Decrypt with:" | cut -d ' ' -f 3-99)
OK=$(eval $CMD | grep "Message:")
if [ "$OK" != "Message: Hello World!" ]; then
    echo "PkEncrypt/PkDecrypt failed :("
    exit -1
fi

CMD=$(target/debug/auth "Hello World!" | grep "Verify with:" | cut -d ' ' -f 3-99)
OK=$(eval $CMD)
if [ "$OK" != "Verified" ]; then
    echo "Auth/Verify failed :("
    exit -1
fi

CMD=$(target/debug/pksign "Hello World!" | grep "Verify with:" | cut -d ' ' -f 3-99)
OK=$(eval $CMD)
if [ "$OK" != "Verified" ]; then
    echo "PkSign/PkVerify failed :("
    exit -1
fi

HASH=$(target/debug/hash "Hello World!")
if [ "$HASH" != "Hash: 35259d2903a1303d3115c669e2008510fc79acb50679b727ccb567cc3f786de3553052e47d4dd715cc705ce212a92908f4df9e653fa3653e8a7855724d366137" ]; then
    echo "Hash failed :("
    exit -1
fi