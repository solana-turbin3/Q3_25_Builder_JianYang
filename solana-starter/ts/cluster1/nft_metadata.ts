import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({address: "https://devnet.irys.xyz"}));
umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        // const image = await readFile('https://gateway.irys.xyz/3xL6m2kF28taQiaQkTprDFVxXM4jDRN9bRU41d3ZqpL1');
        // const image = "https://gateway.irys.xyz/3xL6m2kF28taQiaQkTprDFVxXM4jDRN9bRU41d3ZqpL1";
        const image = "https://gateway.irys.xyz/67CPrEPnYQZK2EiNSWXpCYzAuhzRGYJEhFYqCodyHfVa";

        const metadata = {
            name: "RARERUG_V2",
            symbol: "RRUG2",
            description: "the cool pattern makes this rare",
            image: image,
            attributes: [
                {trait_type: "rare", value: "hmmmmmm"}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://gateway.irys.xyz/67CPrEPnYQZK2EiNSWXpCYzAuhzRGYJEhFYqCodyHfVa"
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
