import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

// Setup on page load
window.onload = async () => {
  let isSignedIn = await wallet.startUp();
console.log(isSignedIn);
  if (isSignedIn == true) {
    signedInFlow();
    console.log("Signing in...");
  } else {
    signedOutFlow();
    console.log("Signing out...");
  }

  getBalance();
};

// Button clicksigned-in-flows
document.querySelector('form#muaCRT').onsubmit = doUserAction;
document.querySelector('#sign-in-button').onclick = () => { wallet.signIn(); };
document.querySelector('#sign-out-button').onclick = () => { wallet.signOut(); };

// Take the new greeting and send it to the contract
async function doUserAction(event) {
  event.preventDefault(); // Ngăn chặn form tự động gửi dữ liệu lên server
  const buyCRTInput = document.getElementById('buyCRT');
  const quantity = buyCRTInput.value;
  console.log('Số lượng CRT đã nhập:', quantity);
  const amount = Number(quantity);
  console.log('Số lượng CRT đã nhập (integer):', amount);

  document.querySelector('#signed-in-flow-2 main')
    .classList.add('please-wait');

  await wallet.callMethod({ method: 'transfer', args: { receiver_id: wallet.accountId.toString(),amount: amount }, contractId: CONTRACT_ADDRESS });

  // // ===== Fetch the data from the blockchain =====
  await getBalance();
  document.querySelector('#signed-in-flow-2 main')
    .classList.remove('please-wait');
}

// Get greeting from the contract on chain
async function getBalance() {
  const currentGreeting = await wallet.viewMethod({ method: 'get_balance',args:{account_id: wallet.accountId.toString()}, contractId: CONTRACT_ADDRESS });

  document.querySelectorAll('[data-behavior=balance]').forEach(el => {
    el.innerText = currentGreeting;
    el.value = currentGreeting;
  });
}

// Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-in-flow').style.display = 'none';
  document.querySelector('#signed-in-flow-2').style.display = 'none';
  document.querySelector('#signed-out-flow').style.display = 'block';
}

// Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-out-flow').style.display = 'none';
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelector('#signed-in-flow-2').style.display = 'block';
  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = wallet.accountId;
  });
}