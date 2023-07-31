
## Smart Contracts

Hợp đồng thông minh là
**tự thực hiện** hợp đồng với các điều khoản của thỏa thuận
giữa người mua và người bán được viết trực tiếp vào các dòng mã.
Mã và các thỏa thuận có trong đó tồn tại trên một mạng
**phân tán**, **phi tập trung** [blockchain](https://itviec.com/blog/blockchain-la-gi/). Hợp đồng thông minh cho phép **giao dịch** đáng tin cậy và
**thỏa thuận** được thực hiện giữa những người khác nhau, ẩn danh
các bên mà không cần một cơ quan trung ương, hệ thống pháp luật, hoặc
cơ chế cưỡng chế bên ngoài. Chúng hiển thị các giao dịch **traceable**,
**transparent**, and **irreversible**.

## Blockchain

Các hợp đồng thông minh trên nền tảng Near được phát triển bằng ngôn ngữ lập trình Rust. Near là một `nền tảng blockchain phi tập trung` có khả năng chạy các hợp đồng thông minh. Các hợp đồng này được triển khai trên blockchain Near và có thể được truy cập từ khắp nơi trên mạng.

Near sử dụng Rust làm ngôn ngữ chính để viết các hợp đồng thông minh. Rust là một ngôn ngữ lập trình hiệu năng cao, an toàn và có hệ sinh thái mạnh mẽ, rất thích hợp cho việc phát triển các ứng dụng blockchain.

Khi triển khai một hợp đồng thông minh trên Near, bạn cần trả tiền cho các thợ đào (validators) của mạng bằng một loại tài sản gọi là `NEAR`, tương tự như các blockchain khác sử dụng Gas để tính phí.

Nếu bạn muốn bắt đầu với việc viết hợp đồng thông minh đầu tiên trên mạng Near, bạn có thể tham khảo tài liệu và tài nguyên của Near và Rust để tiếp tục phát triển ứng dụng của mình.

## Platforms

**High Level Languages** for smart contract
developing:

- [Rust](https://www.rust-lang.org/learn)
- [AssemblyScript](https://www.assemblyscript.org/)

See the differences [here](https://blog.suborbital.dev/assemblyscript-vs-rust-for-your-wasm-app).

**Command Line Development Management Tools** for
creating a DAPP project:

- [NEAR CLI](https://docs.near.org/tools/near-cli)

**Testnode with RPC Interace** for deploying
contracts on a virtual node and make transactions without the need to
be mined:

- [Near testnet](https://explorer.testnet.near.org/)  

**Wallet clients** to connect a ethereum wallet

- [NearWallet](https://wallet.testnet.near.org/)

**Compiler**

Các hợp đồng trong Near Protocol được viết bằng ngôn ngữ Rust và sau đó được biên dịch để tạo mã `Giao diện ứng dụng nhị phân` (ABI). ABI là giao diện giữa hai module chương trình, trong đó một trong số chúng thường ở mức mã máy. Giao diện này quy định cách mã hóa và giải mã dữ liệu vào/ra từ mã máy. Nó xác định cách mã hóa các lệnh gọi hợp đồng Rust cho Near Virtual Machine (NVM) và ngược lại, cũng như cách trích xuất dữ liệu từ các giao dịch. Nó cũng cung cấp mã `Bytecode` hoặc opcodes của hợp đồng.

Một số công cụ dòng lệnh, ví dụ như `near-cli`, cũng như các IDE trực tuyến như `NEAR Studio` có thể được sử dụng để biên dịch và quản lý các hợp đồng trong Near Protocol.

**Network**

Các hợp đồng thông minh trong Near Protocol được triển khai trên mạng Near và thực thi trên Virtual Machine của Near (NearVM). Điều này hỗ trợ việc phát triển hơn vì không yêu cầu sử dụng token giao dịch như Ethereum. Trên mạng Near, có hai loại mạng chính:

Mainnet: Đây là mạng chính thức của Near Protocol, tương tự như 'livenet' của Ethereum. Trên mạng này, bạn sử dụng đồng tiền thực có tên gọi là "NEAR" để thực hiện các giao dịch và triển khai hợp đồng thông minh.

Testnet: Near cũng cung cấp nhiều mạng kiểm tra ('testnet') để phát triển và kiểm thử ứng dụng. Trên các mạng kiểm tra này, bạn nhận được đồng tiền kiểm tra có tên gọi là "test NEAR" miễn phí, không giống như đồng tiền thực NEAR trên mạng chính.

Sự khác biệt chính giữa hai mạng là việc sử dụng đồng tiền NEAR thực (Mainnet) và đồng tiền kiểm tra NEAR (Testnet). Việc có các mạng kiểm tra giúp cho việc phát triển và thử nghiệm ứng dụng trên Near Protocol dễ dàng hơn và không tốn chi phí.

**Wallet**

Ví rất
một phần quan trọng của một hợp đồng thông minh. Nó phục vụ 2 mục đích:

- Nó đóng vai trò là ứng dụng khách cho ví Near. Để thực hiện giao dịch trên mạng near phải được chi tiêu và bạn có thể ủy quyền các khoản thanh toán này bằng cách này.
- Để giao tiếp với một chuỗi khối và để triển khai, bạn cần phải có một nút đầy đủ hoặc một ví của mạng. Một chiếc ví có thể tạo điều kiện giao tiếp với mạng.

**Deployment**

Để triển khai một hợp đồng
các bước sau đây sẽ được thực hiện:

- Biên dịch các
   mã và nhận **bytecodes** và **ABIcodes** cần thiết
- Chọn một
   mạng để di chuyển đến
- Làm một
   triển khai với địa chỉ ví làm người gửi giao dịch
- Xác thực
   giao dịch tạo thành ví và thanh toán chi phí giao dịch.

Hợp đồng của bạn sẽ
được triển khai và sẽ được chỉ định một địa chỉ công cộng có thể được sử dụng
để truy cập nó.

**Web
Interface**

Một ứng dụng web có thể được
đã từng làm việc với hợp đồng. Một backend javascript framework,
**web3.js**, có thể kết hợp với blockchain. Nó có thể kết nối với
mạng, xác định hợp đồng và thực hiện các giao dịch. Ở đó
có hai loại hoạt động giao dịch trên một hợp đồng:

** 1.Call**

"Call" là một cuộc gọi hàm của hợp đồng mà không cần phát sóng hoặc công bố bất cứ điều gì trên blockchain. Đây là một hoạt động chỉ đọc và sẽ không tiêu thụ bất kỳ `NEAR` nào. Nó mô phỏng những gì sẽ xảy ra trong một giao dịch, nhưng khi hoàn thành, tất cả các thay đổi trạng thái sẽ bị loại bỏ. Điều này được thực hiện đồng bộ và giá trị trả về của hàm hợp đồng sẽ được trả lại ngay lập tức.

**2.Transaction**

Một giao dịch được broadcasted đến network, được xử lý bởi các miners/validators, và nếu hợp lệ, sẽ được published trên blockchain. Đây là một hoạt động ghi mà sẽ ảnh hưởng đến các tài khoản khác, cập nhật trạng thái của blockchain và tiêu thụ Near (trừ khi một miners/validators chấp nhận nó với giá Gas là 0). Điều này diễn ra không đồng bộ, bởi vì có thể không có miners/validators nào bao gồm giao dịch trong một khối (ví dụ, giá Gas cho giao dịch có thể quá thấp). Vì nó diễn ra không đồng bộ, giá trị `trả về ngay lập tức của một giao dịch luôn là băm của giao dịch (transaction hash)`.

**Web3js framework** hoạt động như sau:

- Kết nối với một mạng bằng cách sử dụng 'web3Provider' đến một localhost (mạng thử nghiệm cục bộ) hoặc một mạng toàn cầu.
- Tạo một đối tượng hợp đồng (contract instance) bằng cách sử dụng mã ABI (Application Binary Interface) và địa chỉ hợp đồng. Địa chỉ hợp đồng xác định hợp đồng cụ thể trên mạng mà chúng ta muốn tương tác và mã ABI xác định cách truy cập vào từng chức năng của hợp đồng.
- Sử dụng đối tượng hợp đồng để gọi các chức năng của hợp đồng giống như sử dụng javascript.

<!-- **Steps:**

**Install MetaMask**

1. Tạo tài khoản trên [Near wallet testnet](https://wallet.testnet.near.org/).

2. Setup a **password** and open the wallet. Select the network
   as ‘**Rinkeby Test Network**’.

3. Click on ‘**CREATE ACCOUNT**’ to create a new wallet
   accout and click ‘**Copy Address to clipboard**’ to copy your
   **public address** for the wallet.

4. Go to [https://faucet.rinkeby.io/](https://faucet.rinkeby.io/)
   to get free test ether to the address. Check your account on metamask
   and verify the **balance**.

5. Repeat steps 3 and 4 to create more accounts.

**Deploying contract**

1. Go to [http://remix.ethereum.org/](http://remix.ethereum.org/)
   and **upload** your contract file (**Ballot.sol**)

2. **Compile** the code. Make sure you’ve slected ‘**Ballot.sol**’
   in the dropdown next to details. Ignore warnings.

3. Go to the **run** tab. Make sure ‘**Environment**’ is
   set as ‘**Injected Web3** ’ and shows ‘**rinkeby**’.
   Make sure ‘**Account**’ shows your wallet address in metamask.
   This is the account from which the contract will be delpoyed. ‘Gas
   limit’ and ‘Value’ has little importance on testnet but make
   sure to pay enough gas on livenet.

4. Make sure ‘**Ballot**’ is shown in the dropdown above
   ‘**create**’

(If any of the above steps fail, reload the browser)

5. click ‘**create**’ and a **popup** will appear on
   metamask. Open metamask and **Submit** the transaction. Set a
   reasonable ‘Gas limit’ and ‘Gas Price’ according to network.

6. Click on the transaction to go to
   [https://rinkeby.etherscan.io/tx/](https://rinkeby.etherscan.io/tx/)
   to know the status of transaction. If it is a **success**, your
   contract is deployed. In the ‘**To**’ section **“[Contract
   0x0000000000000000000000000000000000
   Created]”** will be shown. This is your **contract address**.
   Copy it. Click on it to know about the incoming transaction to the
   contract.

Now the contract is deployed on the rinkeby network. You can
access it using a web app. -->

<!-- **Web App**

1. Open **src/repository/services.ts** file. This is the typescript file
   that interacts with the contract.

2. Change the configuration file with contract details at
   **src/repository/config.tsx**

3. Go to remix page.
   In the **compile** section go to **details** tab. In the **ABI**
   section click on copy button to copy your ABI code.

4. Go to
   **src/repository/KYC.json** file and paste it replacing entire json

5. Run
   **npm install && npm start** to open the web app. -->

<!-- **Interacting
on web App**

Fetching details
from a contract is a ‘**call**’ transaction and would’nt be
send as a transaction from metamask.

A user making a
transaction to contract is identified by his wallet address. Make
sure to switch metamask accounts before making the transaction. Only
the address from which the contract was deployed will be able to
perform certain operations. Refer the **Contract Defenition** file
for more information. -->

# Smart-Contracts

### Requirements

- Cargo: cargo 1.69.0
- Compiler: rustc: 1.69.0
- Near-sdk: v4.1.1
- NodeJS v12 or later
- Windows, Linux or Mac OS X

### Set-up

```
Language => rust
Tool => Near CLI
Network interface => near testnet
```
