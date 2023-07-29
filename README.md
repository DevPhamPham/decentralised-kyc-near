# Quickstart

Clone this repository locally or [**open it in gitpod**](https://github.com/DevPhamPham/CharityChain-CRT-). Then follow these steps:

### 1. Install Dependencies
```bash
npm install
```

### 2. Test the Contract
Deploy your contract in a sandbox and simulate interactions from users.

```bash
npm test
```

### 3. Deploy the Contract
Build the contract and deploy it in a testnet account
```bash
npm run deploy
```

### 4. Start the Frontend
Start the web application to interact with your smart contract 
```bash
npm start
```

---

# Decentralised KYC Near

KYC là một quá trình mà các ngân hàng có được thông tin về danh tính và địa chỉ của người mua. Đó là quy trình do cơ quan quản lý điều hành nhằm thực hiện thẩm định để xác minh danh tính của khách hàng. Quá trình này giúp đảm bảo rằng các dịch vụ của ngân hàng không bị lạm dụng. Các ngân hàng có trách nhiệm hoàn thành thủ tục KYC khi mở tài khoản. Các ngân hàng cũng được yêu cầu cập nhật định kỳ chi tiết KYC của khách hàng. KYC có thể là thủ công, tốn thời gian và dư thừa giữa các tổ chức. Chia sẻ thông tin KYC trên Blockchain sẽ cho phép các tổ chức tài chính mang lại kết quả tuân thủ tốt hơn, tăng hiệu quả và cải thiện trải nghiệm của khách hàng.

## Vấn đề

Mỗi công ty phải xác minh danh tính của bạn bằng cách nào đó và điều này đặc biệt quan trọng đối với các tổ chức tài chính. Từ đó, các giao thức 'biết khách hàng của bạn' hay KYC đã phát triển để hỗ trợ các công ty đảm bảo họ biết họ đang kinh doanh với ai. Thông thường, điều này liên quan đến một phương pháp mở rộng, rút ra trong đó một số tài liệu nhất định được hiển thị và một số loại kiểm tra hoặc xác minh lý lịch diễn ra. Trong hệ thống KYC truyền thống, mỗi ngân hàng sẽ tiến hành kiểm tra danh tính của mình, tức là mỗi người dùng được kiểm tra riêng lẻ bởi một tổ chức cá nhân hoặc cơ cấu chính phủ. Do đó, sẽ lãng phí thời gian để kiểm tra từng danh tính từ đầu.

## Giải pháp

Kiến trúc chuỗi khối và DLT cho phép nhóm thu thập thông tin từ các nhà cung cấp dịch vụ khác nhau vào một cơ sở dữ liệu không thay đổi và bảo mật bằng mật mã mà không cần bên thứ ba xác minh tính xác thực của kiến thức. Có thể hình thành một hệ thống mà người dùng chỉ cần thực hiện quy trình KYC một lần để xác minh danh tính của mình.

## Required:

- Các vai trò khác nhau: Tổ chức tài chính quản trị (ví dụ: RBI), Tổ chức tài chính & Khách hàng
- Smart contract bao gồm tất cả các quy tắc và giao thức cần thiết cho luồng tài liệu KYC. Nhóm đã tạo 2 địa chỉ liên hệ cho Ngân hàng và Khách hàng, đồng thời kế thừa hợp đồng KYC của hợp đồng đó.
- Mạng Blockchain để triển khai Hợp đồng. Nhóm đã sử dụng Near Testnet cho hợp đồng của này.

## Các giả định:

<b>1. Admin của tổ chức tài chính có thể thêm FIs(Financial Institutions - tổ chức tài chính) đã được xác minh:</b>

        Người quản trị của tổ chức tài chính có thể thêm các tổ chức tài chính khác đã được xác minh vào hệ thống.

<b>2. Admin có thể làm FIs trở nên hoạt động/không hoạt động liên quan đến bất kỳ hành động nào:</b>

        Người quản trị có quyền làm cho các tổ chức tài chính trở thành hoạt động hoặc không hoạt động liên quan đến các chức năng và hoạt động trong hệ thống.

<b>3. FIs có thể thêm Khách hàng và yêu cầu KYC từ Khách hàng:</b>

        Các tổ chức tài chính có thể thêm khách hàng vào hệ thống và yêu cầu khách hàng cung cấp thông tin KYC.

<b>4. Khách hàng có thể phê duyệt/từ chối yêu cầu KYC từ FI:</b>

        Khách hàng có quyền phê duyệt hoặc từ chối yêu cầu cung cấp thông tin KYC từ tổ chức tài chính.

<b>5. Nếu Khách hàng chấp thuận yêu cầu KYC, một thông báo (qua email/số điện thoại) sẽ được gửi đến FI và FI có thể truy cập các tài liệu KYC của khách hàng như Thẻ Aadhar, Pancard, ID ảnh, Chữ ký, v.v. để xác minh:</b>

        Nếu khách hàng đồng ý cung cấp thông tin KYC, một thông báo sẽ được gửi đến tổ chức tài chính, thông qua email hoặc số điện thoại, và tổ chức tài chính có quyền truy cập các tài liệu KYC của khách hàng như Chứng minh nhân dân Aadhar, Chứng minh thuế PAN, Giấy tờ tùy thân, Chữ ký, v.v. để tiến hành xác minh.

<b>6. FIs có thể phê duyệt/từ chối dữ liệu của Khách hàng sau khi xác minh:</b>

        Nếu tổ chức tài chính từ chối xác minh KYC của khách hàng, một thông báo sẽ được gửi đến khách hàng qua email hoặc số điện thoại, kèm theo lý do từ chối.

<b>7. Nếu FI từ chối xác minh KYC của Khách hàng, một thông báo (qua email/số điện thoại) sẽ được gửi cho Khách hàng kèm theo lý do:</b>

        Nếu tổ chức tài chính từ chối xác minh KYC của khách hàng, một thông báo sẽ được gửi đến khách hàng qua email hoặc số điện thoại, kèm theo lý do từ chối.

<b>8. Khách hàng có thể cập nhật các tài liệu KYC và thông báo cập nhật sẽ được kích hoạt cho tất cả các FIs được kết nối:</b>

        Nếu tổ chức tài chính từ chối xác minh KYC của khách hàng, một thông báo sẽ được gửi đến khách hàng qua email hoặc số điện thoại, kèm theo lý do từ chối.

<b>9. Tất cả các vai trò người dùng phải có địa chỉ metamask bắt buộc trên mạng triển khai.</b>

<b>10. Người dùng triển khai hợp đồng lên mạng chính sẽ được coi là Quản trị viên FI:</b>

        Người dùng triển khai hợp đồng thông minh (smart contract) lên mạng chính sẽ được xem là quản trị viên của tổ chức tài chính.