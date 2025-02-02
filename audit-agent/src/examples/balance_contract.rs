pub const CONTRACT_CODE: &str = r#"
    pragma solidity ^0.8.0;
    
    contract Vulnerable {
        mapping(address => uint) private balances;
        
        function withdraw(uint amount) public {
            require(balances[msg.sender] >= amount);
            (bool success, ) = msg.sender.call{value: amount}("");
            require(success);
            balances[msg.sender] -= amount;
        }
    }
    "#;
