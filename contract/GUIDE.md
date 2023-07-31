#### Init admin:

    near call `contract_id` new '{"name_":"Pham Duy Khoa","email_":"pham.duykhoa1303@gmail.com"}' --accountId `your_account_id`

#### Add CO(thêm một tổ chức):

     near call `contract_id` add_co_kyc '{"co": {"name": "Charity Organization", "email": "charity@example.com", "id_": "co_account_id", "npoid_code": "12345", "kyc_count": 0, "status": "Active"}}' --accountId `your_account_id`

#### Get all CO:

    near call `contract_id` get_all_co_kyc '{"page_number":1}' --accountId `your_account_id`

##### result demo:

    [
        1,
        [
            {
            name: 'Charity Organization',
            email: 'charity@example.com',
            id_: 'co_account_id',
            npoid_code: '12345',
            kyc_count: 0,
            status: 'Active'
            }
        ]
    ]
    