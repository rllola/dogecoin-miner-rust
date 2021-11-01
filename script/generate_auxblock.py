from scrypt_auxpow import *

hash = "d4d75826efa5b8b96766aeca2b19025101d2bdf5f4a9ec550bebdbad0b86b331"
target = "7fffff0000000000000000000000000000000000000000000000000000000000"
id = "00"


apow = computeAuxpowWithChainId(hash, target, id, True)

print(apow)