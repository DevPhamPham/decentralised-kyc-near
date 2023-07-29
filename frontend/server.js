const nearApi = require('near-api-js');
const { setupModal } = require('@near-wallet-selector/modal-ui');
const { setupWalletSelector } = require('@near-wallet-selector/core');
const { setupLedger } = require('@near-wallet-selector/ledger');
const { setupMyNearWallet } = require('@near-wallet-selector/my-near-wallet');
const THIRTY_TGAS = '30000000000000';
const NO_DEPOSIT = '0';

class Wallet {
  constructor({ createAccessKeyFor = undefined, network = 'testnet' }) {
    this.createAccessKeyFor = createAccessKeyFor;
    this.network = network;
  }

  async startUp() {
    this.walletSelector = await setupWalletSelector({
      network: this.network,
      modules: [setupMyNearWallet({ iconUrl: '@near-wallet-selector/my-near-wallet/assets/my-near-wallet-icon.png' }), setupLedger({ iconUrl: '@near-wallet-selector/ledger/assets/ledger-icon.png' })],
    });

    const isSignedIn = this.walletSelector.isSignedIn();

    if (isSignedIn) {
      this.wallet = await this.walletSelector.wallet();
      this.accountId = this.walletSelector.store.getState().accounts[0].accountId;
    }

    return isSignedIn;
  }

  signIn() {
    const description = 'Please select a wallet to sign in.';
    const modal = setupModal(this.walletSelector, { contractId: this.createAccessKeyFor, description });
    modal.show();
  }

  signOut() {
    this.wallet.signOut();
    this.wallet = this.accountId = this.createAccessKeyFor = null;
    process.exit(0);
  }

  async viewMethod({ contractId, method, args = {} }) {
    const { network } = this.walletSelector.options;
    const provider = new nearApi.providers.JsonRpcProvider({ url: network.nodeUrl });

    let res = await provider.query({
      request_type: 'call_function',
      account_id: contractId,
      method_name: method,
      args_base64: Buffer.from(JSON.stringify(args)).toString('base64'),
      finality: 'optimistic',
    });
    return JSON.parse(Buffer.from(res.result).toString());
  }

  async callMethod({ contractId, method, args = {}, gas = THIRTY_TGAS, deposit = NO_DEPOSIT }) {
    const outcome = await this.wallet.signAndSendTransaction({
      signerId: this.accountId,
      receiverId: contractId,
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName: method,
            args,
            gas,
            deposit,
          },
        },
      ],
    });

    return nearApi.providers.getTransactionLastResult(outcome);
  }

  async getTransactionResult(txhash) {
    const { network } = this.walletSelector.options;
    const provider = new nearApi.providers.JsonRpcProvider({ url: network.nodeUrl });

    const transaction = await provider.txStatus(txhash, 'unnused');
    return nearApi.providers.getTransactionLastResult(transaction);
  }
}

module.exports = {
  Wallet,
};
