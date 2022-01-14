//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.10;

contract Addition {
    int public x;

    function add(uint a, uint b) public pure returns(uint) {
        return a + b;
    }
}