const express = require('express');
const crypto = require('crypto');
const solanaWeb3 = require('@solana/web3.js');
const nacl = require('tweetnacl');
const fs = require('fs');
var bs58 = require('bs58');
const path = require('path');
const jwt = require('jsonwebtoken');
const bodyParser = require("body-parser");

const app = express();
const port = 3000;
const connection = new solanaWeb3.Connection(
    solanaWeb3.clusterApiUrl('devnet'),
    'confirmed'
);

app.use(express.static(__dirname));
app.use(express.json());
app.use(bodyParser.json());

app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname + '/index.html'));
});

// GET route to retrieve a nonce value for use in signing
app.get('/api/nonce', (req, res) => {
    console.log("Nonce: ")
    // Generate a random 32-byte value to use as the nonce
    const nonce = crypto.randomBytes(32).toString('hex');
    console.log(nonce)
    // Return the nonce value as a JSON object in the response body
    res.json({ nonce });
});

const secretKey = 'mySecretKey';


app.post('/login', (req, res) => {
    const { signedMessage, message, publicKey } = req.body;

    const uint8Array = bs58.decode(publicKey);
    const array = Array.from(uint8Array);
    const decodedPublicKey = Buffer.from(array);

    let signature;
    try {
        signature = Buffer.from(signedMessage, 'base64');
    } catch (error) {
        res.status(400).send('The signed message is not a valid base64 string');
        return;
    }

    const messageBuffer = Buffer.from(message, 'utf8');

    console.log(signedMessage.length);

    const isValid = nacl.sign.detached.verify(
        Uint8Array.from(messageBuffer),
        Uint8Array.from(signature),
        Uint8Array.from(decodedPublicKey)
    );

    console.log("isValid: " + isValid)

    if (!isValid) {
        return res.status(401).json({ error: 'Invalid signature.' });
    }

    const token = jwt.sign({ publicKey }, secretKey, { expiresIn: '1h' });
    console.log(token);

    res.json(token); // Send JWT to the frontend
});


// Endpoint for verifying the JWT token and logging in the user
app.post('/verify', (req, res) => {
    const authHeader = req.headers.authorization;
    console.log("authHeader: " + authHeader);

    if (!authHeader || !authHeader.startsWith('Bearer ')) {
        return res.status(401).json({ error: 'Invalid token' });
    }

    const token = authHeader.split(' ')[1];
    console.log(token);

    try {
        // Verify the JWT token
        const decoded = jwt.verify(token, secretKey);
        console.log(decoded);
        const currentTime = Math.floor(Date.now() / 1000);
        console.log(currentTime);
        if (decoded.exp < currentTime) {
            res.status(200).json({ message: "tokenExpired" });
        } else {
            res.status(200).json({ message: "ok" });
        }

    } catch (err) {
        res.status(401).json({ error: 'Invalid token' });
    }
});

app.post('/adduser', async (req, res) => {
    const { programId, newAccountSecretKey, name, profilePhoto, bio } = req.body;
    const data = Buffer.from([1, ...Buffer.from(name.padEnd(32, '\0')), ...Buffer.from(profilePhoto.padEnd(32, '\0')), ...Buffer.from(bio.padEnd(32, '\0'))]);
    const newAccountPublicKey = solanaWeb3.PublicKey.fromSecretKey(Buffer.from(newAccountSecretKey, 'hex'));
    const program = new solanaWeb3.PublicKey(programId);
    const instruction = new solanaWeb3.TransactionInstructicon({
        keys: [{pubkey: newAccountPublickey, isSigner: true, isWritable: true}],
        programld: program,
        data: data,
    });
    let transaction = new solanaWeb3.Transaction().add(instruction);
    transaction.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
    let signingKey = solanaWeb3.Keypair.fromSecretKey(new Uint8Array(Buffer.from(newAccountSecretKey, 'hex')));
    transaction.sign(signingKey);
    let rawTransaction = transaction.serialize();
    let txid = await connection.sendRawTransaction(rawTransaction);
    res.json({txid});
});

app.post('/followUser', async (req, res) => {
    const { programId, followeSecretKey, followedPublicKey } = req.body;
    const data = Buffer.from([2, ...new solanaWeb3.PublicKey(followedPublicKey).toBytes()]);
    const followerPublicKey = solanaWeb3.PublicKey.fromSecretKey(Buffer.from(followerSecretKey, 'hex'));
    const instruction = new solanaWeb3.TransactionInstruction({
    keys: [{pubkey: followerPublicKey, isSigner: true, isWritable: true}],
    programId: new solanaWeb3.PublicKey(programId),
    data: data,
    });
    let transaction = new solanaWeb3.Transaction().add(instruction);
    transaction.recentBlockhash = (await connectiom.getRecentBlockhash()).blockhash;
    let signingKey = solanaWeb3.Keypair.fromSecretKey(new Uint8Array(Buffer.from(followerSecretKey, 'hex')));
    transaction.sign(signingKey);
    let rawTransaction = transaction.serialize();
    let txid = await connection.sendRawTransaction(rawTransaction);
    res.json({txid});
});

// Serve the success page
app.get('/success', (req, res) => {
    res.sendFile(path.join(__dirname + '/home.html'));
});

app.listen(port, () => {
    console.log(`Server started on port ${port}`);
});
