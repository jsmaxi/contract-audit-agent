pub const _OTHER_CONTRACT_CODE: &str = r#"
    #[contract]
    mod TokenPool {
        use starknet::ContractAddress;
        use starknet::get_caller_address;

        #[storage]
        struct Storage {
            balances: LegacyMap::<ContractAddress, u256>,
            token: ContractAddress,
        }

        #[constructor]
        fn constructor(ref self: ContractState, token_address: ContractAddress) {
            self.token.write(token_address);
        }

        #[external(v0)]
        fn deposit(ref self: ContractState, amount: u256) {
            let caller = get_caller_address();
            IERC20Dispatcher::transfer_from(
                self.token.read(),
                caller,
                get_contract_address(),
                amount
            );
            self.balances.write(caller, self.balances.read(caller) + amount);
        }

        #[external(v0)]
        fn withdraw(ref self: ContractState, amount: u256) {
            let caller = get_caller_address();
            let current_balance = self.balances.read(caller);
            IERC20Dispatcher::transfer(self.token.read(), caller, amount);
            self.balances.write(caller, current_balance - amount);
        }
    }
    "#;
