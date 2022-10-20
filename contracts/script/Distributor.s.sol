// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/console2.sol";
import "forge-std/Script.sol";
import {Distributor} from "../src/Distributor.sol";

contract DistributorScript is Script {
    function setUp() public {}

    function deploy() public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        console2.log("Distributing from", vm.addr(privateKey));
        console2.log("Starting balance: ", vm.addr(privateKey).balance);
        vm.startBroadcast(privateKey);
        Distributor distributor = new Distributor{salt: 0}();
        console2.log("Deployed Distributor at", address(distributor));
    }

    function distribute(uint256 amountPerAddress, string memory prefix, uint256 numFiles) public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        console2.log("Distributing from", vm.addr(privateKey));
        console2.log("Starting balance: ", vm.addr(privateKey).balance);

        Distributor distributor = Distributor(address(0xe2a453EAc17001f311F642976509E8C502138756));

        vm.startBroadcast(privateKey);
        for (uint256 i = 0; i < 1; i++) {
            string memory path = "./addresses/";
            path = string.concat(path, prefix);
            path = string.concat(path, "_");
            path = string.concat(path, vm.toString(i));
            path = string.concat(path, ".json");
            // suuuper slow
            address[] memory addresses = abi.decode(vm.parseJson(vm.readFile(path)), (address[]));
            distributor.distribute{value: amountPerAddress * addresses.length}(addresses, amountPerAddress);
        }
    }
}
