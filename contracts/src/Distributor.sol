// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Distributor {
    function distribute(address[] memory addresses, uint256 amountPerAddress) external payable {
        for (uint256 i = 0; i < addresses.length; i++) {
            // they're all EOAs so normal transfer is fine
            payable(addresses[i]).transfer(amountPerAddress);
        }

        // send back any remainder
        payable(msg.sender).transfer(address(this).balance);
    }
}
