# User Managerment Tool using Rust 
[![Rust](https://github.com/arunkumar-mourougappane/rust-user-managerment-crud/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/arunkumar-mourougappane/rust-user-managerment-crud/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/arunkumar-mourougappane/rust-user-managerment-crud/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/arunkumar-mourougappane/rust-user-managerment-crud/actions/workflows/rust-clippy.yml)

A simple application developed as part of learning rust programming language while interface with PostgreSQL.

## Minimal UI to add and manager users:

![image](https://github.com/user-attachments/assets/3c09efa5-5895-40ac-b438-d5cd21d6f434)

## Addding Users:
Adding  a user would require to input `1` to the interface.

![image](https://github.com/user-attachments/assets/8d6a950e-637d-4358-8b38-9d3604e99342)

You then end up adding user infomation. The User Account ID is auto generated and is printed back on successful creattion.

## Viewling User(s):
### View User:
An `account ID` is needed to get user information from the database. This when option when selected is prompted for account id information
and if the user account if found is printed.

![image](https://github.com/user-attachments/assets/cb2c49ab-4cbd-4638-a3fb-a86e6032f5ee)

### View all Users:
This menu option when selected, prints all user infomation in table format.
![image](https://github.com/user-attachments/assets/f1a88ab2-27b1-49b0-9b2e-4a2050548b26)

## Update User Information:
On selecting user options, you will be prompted for `account id`. if account is found the user information is printed and the editable
fields are prompted for.

![image](https://github.com/user-attachments/assets/b51feda0-dc72-44bf-8a22-2f3b4ed57d10)

On providing information, the user information is updated in database and, prints updated information from database.

![image](https://github.com/user-attachments/assets/9cd3fd19-9ea9-4484-94ef-9a022b9d4c30)

## Activate/Deactivate User:
On selecting option, UI prompts for `account id`. When a User is found in database, you can activate or deactivate by following the prompt.

### Activate User Account:
![image](https://github.com/user-attachments/assets/f3337392-5c76-4bf9-93e2-a78b3d72224c)

### Deactivate User Account:
![image](https://github.com/user-attachments/assets/11c3848a-3acc-43dd-a5d2-c03ad8854930)

