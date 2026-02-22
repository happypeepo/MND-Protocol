// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract LatticeGateway {
    address public immutable targetLogic;

    constructor(address _targetLogic) {
        targetLogic = _targetLogic;
    }

    /// @notice Unpacks the Lattice-compressed payload entirely in EVM memory
    /// @dev First byte is Dictionary ID (0x01 = transfer). 
    fallback(bytes calldata compressedPayload) external returns (bytes memory) {
        address target = targetLogic;
        
        assembly {
            // Retrieve the free memory pointer
            let decompressedMem := mload(0x40) 
            
            // Read first byte (Dictionary ID) directly from calldata offset 0
            // We shift right by 248 bits to isolate the single byte
            let dictId := shr(248, calldataload(0))
            
            let payloadOffset := 1 // Skip the 1-byte ID
            let argsLength := sub(calldatasize(), 1) 
            
            // 0x01 maps to 0xa9059cbb [transfer(address,uint256)]
            if eq(dictId, 0x01) {
                // Shift the 4-byte selector directly into memory (left by 224 bits)
                mstore(decompressedMem, shl(224, 0xa9059cbb))
                
                // Copy the remaining calldata arguments directly behind the selector
                calldatacopy(add(decompressedMem, 4), payloadOffset, argsLength)
                
                // Update length for the underlying logic call (4 bytes selector + args)
                argsLength := add(argsLength, 4)
            }
            
            // Bypass mapping if dictId doesn't exist
            if iszero(eq(dictId, 0x01)) {
                calldatacopy(decompressedMem, 0, calldatasize())
                argsLength := calldatasize()
            }

            // Execute the delegatecall natively using the decompressed memory footprint
            let success := delegatecall(gas(), target, decompressedMem, argsLength, 0, 0)
            
            // Forward return data accurately
            returndatacopy(0, 0, returndatasize())
            if iszero(success) {
                revert(0, returndatasize())
            }
            return(0, returndatasize())
        }
    }
}
