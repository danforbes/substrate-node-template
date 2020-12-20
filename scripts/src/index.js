import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

import { addAttribute, deleteAttribute, getHashForAttribute, getNonceForAttribute, readAttribute, revokeAttribute, types } from './lib.js';

async function main () {
  const provider = new WsProvider("ws://127.0.0.1:9944");
  const api = await ApiPromise.create({ provider, types });
  const keyring = new Keyring({ type: 'sr25519' });

  const users = {
    alice: { key: keyring.addFromUri('//Alice', { name: 'Alice' }), nonce: 0 },
    bob: { key: keyring.addFromUri('//Bob', { name: 'Bob' }), nonce: 0 },
    bobBank: { key: keyring.addFromUri('//Bob//stash', { name: 'Bob-BANK' }), nonce: 0 },
    charlie: { key: keyring.addFromUri('//Charlie', { name: 'Charlie' }), nonce: 0 },
    charlieBank: { key: keyring.addFromUri('//Charlie//stash', { name: 'Charlie-BANK' }), nonce: 0 },
    dave: { key: keyring.addFromUri('//Dave', { name: 'Dave' }), nonce: 0 },
    daveBank: { key: keyring.addFromUri('//Dave//stash', { name: 'Dave-BANK' }), nonce: 0 },
    eve: { key: keyring.addFromUri('//Eve', { name: 'Eve' }), nonce: 0 },
    eveBank: { key: keyring.addFromUri('//Eve//stash', { name: 'Eve-BANK' }), nonce: 0 },
    ferdie: { key: keyring.addFromUri('//Ferdie', { name: 'Ferdie' }), nonce: 0 },
    ferdieBank: { key: keyring.addFromUri('//Ferdie//stash', { name: 'Ferdie-BANK' }), nonce: 0 },
  }

  await addAttribute(api, users.alice, users.alice.key.address, 'Name', 'Alice');
  await revokeAttribute(api, users.alice, users.alice.key.address, 'Name');
  const attrNonce = await getNonceForAttribute(api, users.alice.key.address, 'Name');
  console.log(getHashForAttribute(api, users.alice.key.address, 'Name', attrNonce.subn(1)).toHex());

  // const blockTime = api.consts.timestamp.minimumPeriod.muln(2).toNumber();

  // let nameAttr = await readAttribute(api, users.alice.key.address, 'Name');
  // if (nameAttr.value) {
  //   await deleteAttribute(api, users.alice, users.alice.key.address, 'Name');
  // }

  // await addAttribute(api, users.alice, users.alice.key.address, 'Name', 'Alice', 3);
  // nameAttr = await readAttribute(api, users.alice.key.address, 'Name');
  // console.log(`${nameAttr}`);
}


main().catch(console.error).finally(() => process.exit());
