// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// Dummy token logic for the delegatecall target
contract LogicToken {
    mapping(address => uint256) public balanceOf;
    
    function transfer(address to, uint256 amount) external {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
    }
    
    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}
