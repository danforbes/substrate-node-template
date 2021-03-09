const polkadot = require('@polkadot/api');
const ApiPromise = polkadot.ApiPromise;
const WsProvider = polkadot.WsProvider;
const Keyring = polkadot.Keyring;

exports.handler = async () => {
    try {
        const wsProvider = new WsProvider("ws://127.0.0.1:9944");
        const api = await ApiPromise.create({ provider: wsProvider, types: { Address: "AccountId", LookupSource: "AccountId", ClassId: "u64", TokenId: "u64" } });
        const keyring = new Keyring({ type: "sr25519" });
        const alice = keyring.addFromUri("//Alice");
        const unsub = await api.tx.chiba.createCollection().signAndSend(alice, (result) => {
            if (result.status.isInBlock || result.status.isFinalized) {
                result.events.forEach(({ phase, event: { data, method, section } }) => {
                    console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
                });
    
                unsub();
            }
        });
    } catch (e) {
        console.error(e);
    }
}
