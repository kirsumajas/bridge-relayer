// make_relayer_key.js
const nacl = require('tweetnacl');

// Generate an Ed25519 keypair
const kp = nacl.sign.keyPair();

// secretKey is 64 bytes: [32-byte secret seed || 32-byte public key]
const skB64 = Buffer.from(kp.secretKey).toString('base64');
const pkHex = Buffer.from(kp.publicKey).toString('hex');

console.log('RELAYER_SK_BASE64=' + skB64);
console.log('RELAYER_PUBKEY_HEX=' + pkHex);
