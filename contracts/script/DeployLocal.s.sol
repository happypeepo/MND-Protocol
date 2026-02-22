// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/LatticeGateway.sol";
import "../src/LogicToken.sol";

contract DeployLocal is Script {
    function run() external {
        // Default Anvil account 0 private key if none provided
        uint256 deployerPrivateKey = vm.envOr("PRIVATE_KEY", uint256(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80));
        
        vm.startBroadcast(deployerPrivateKey);

        LogicToken logic = new LogicToken();
        LatticeGateway gateway = new LatticeGateway(address(logic));
        
        // Seed user A for testing
        LogicToken(address(gateway)).mint(address(0xA1), 500 ether);

        console.log("LOGIC_ADDR:", address(logic));
        console.log("GATEWAY_ADDR:", address(gateway));

        vm.stopBroadcast();
    }
}
