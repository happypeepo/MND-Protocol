// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/LatticeGateway.sol";
import "../src/LogicToken.sol";

contract LatticeGatewayTest is Test {
    LatticeGateway public gateway;
    LogicToken public logic;

    address public userA = address(0xA1);
    address public userB = address(0xB2);

    function setUp() public {
        logic = new LogicToken();
        gateway = new LatticeGateway(address(logic));
        
        // Seed the initial state directly into the proxy's storage 
        // by routing the mint call through the gateway
        LogicToken(address(gateway)).mint(userA, 500 ether);
    }

    function test_YulDecompressionAndState() public {
        uint256 amount = 50 ether;
        
        // --- 1. The Lattice Payload Generation ---
        // Dictionary ID 0x01 + address (padded to 32 bytes) + amount
        bytes memory compressedPayload = abi.encodePacked(
            uint8(0x01),
            uint256(uint160(userB)),
            amount
        );

        // --- 2. Isolate native execution cost ---
        vm.pauseGasMetering();
        vm.prank(userA);
        vm.resumeGasMetering();

        uint256 gasBefore = gasleft();
        (bool success, ) = address(gateway).call(compressedPayload);
        uint256 gasUsed = gasBefore - gasleft();
        
        require(success, "Lattice Dictionary Expansion Failed");

        // --- 3. Prove State Equivalence ---
        (, bytes memory resA) = address(gateway).staticcall(abi.encodeWithSignature("balanceOf(address)", userA));
        (, bytes memory resB) = address(gateway).staticcall(abi.encodeWithSignature("balanceOf(address)", userB));
        
        assertEq(abi.decode(resA, (uint256)), 450 ether, "User A balance did not correctly decrement");
        assertEq(abi.decode(resB, (uint256)), 50 ether, "User B balance did not correctly increment");

        // --- 4. Hackathon Log Output ---
        console.log("========================================");
        console.log("State Integrity: VERIFIED");
        console.log("Memory Yul Decompression Gas Cost:", gasUsed);
        console.log("========================================");
    }
}
