export function addAttribute (api, sender, did, attrName, attrVal, validFor) {
  return sendSignedTxn(api, api.tx.decentralizedIdentity.addAttribute(did, attrName, attrVal, validFor), sender);
}

export async function readAttribute (api, did, attrName) {
  const nonce = await api.query.decentralizedIdentity.attributeNonce([did, attrName]);
  if (nonce.eq(0)) {
    return {};
  }

  const attrHash = getHashForAttribute(api, did, attrName, nonce.subn(1));
  return api.query.decentralizedIdentity.attributeOf([did, attrHash]);
}

export function getNonceForAttribute (api, did, attrName) {
  return api.query.decentralizedIdentity.attributeNonce([did, attrName]);
}

export function getHashForAttribute (api, did, attrName, nonce) {
  return api.registry.createType('(AccountId, Text, u64)', [did, attrName, nonce]).hash;
}

export function deleteAttribute (api, sender, did, attrName) {
  return sendSignedTxn(api, api.tx.decentralizedIdentity.deleteAttribute(did, attrName), sender);
}

export function revokeAttribute (api, sender, did, attrName) {
  return sendSignedTxn(api, api.tx.decentralizedIdentity.revokeAttribute(did, attrName), sender);
}

export async function sendSignedTxn (api, txn, sender) {
  if (!sender.nonce) {
    const accountInfo = await api.query.system.account(sender.key.address);
    sender.nonce = accountInfo.nonce.toNumber();
  }

  const txnId = `${sender.key.meta.name}-${sender.nonce}`;
  const getType = (arg) => `${arg.type}` === 'Bytes' && arg.Type.name === 'Text' ? 'Text' : arg.type;
  const args = txn.args.map((arg, idx) => `${api.registry.createType(getType(txn.meta.args[idx]), arg)}`);
  console.log(` > [${txnId}] Submitting: ${txn.method.section}.${txn.method.method}(${args})`);
  return new Promise(async (resolve, reject) => {
    try {
      const drop = await txn.signAndSend(sender.key, { nonce: sender.nonce++ }, ({ status, events, dispatchError }) => {
        if (!status.isInBlock && !status.isFinalized) {
          return;
        }

        drop();
        if (dispatchError) {
          if (!dispatchError.isModule) throw `${dispatchError}`;
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          throw decoded.documentation.join(' ');
        }

        console.log(` < [${txnId}] In block: ${status.asInBlock}`);
        resolve(events);
      });
    } catch (e) {
      reject(`${e}`);
    }
  });
}

export const types = {
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "Signature": "Vec<u8>",
  "Attribute": {
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "BlockNumber",
    "creation": "Moment",
    "nonce": "u64"
  },
  "AttributeTransaction": {
    "signature": "Signature",
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "u32",
    "signer": "AccountId",
    "identity": "AccountId"
  },
}
