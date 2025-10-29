// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IMailbox} from "hyperlane-monorepo/solidity/contracts/Mailbox.sol";

contract Counter {
    uint256 public number;
    IMailbox public mailbox;

    constructor() {
        mailbox = IMailbox(0x54148470292C24345fb828B003461a9444414517);
    }

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }

    function send() public {
        uint32 destination = 1399811150;
        // bytes32 recipient = bytes32(abi.encodePacked("4N8f8zmwb3cuT8wMYAbEqRw47UxrLfwk3HXiCmJMwmvu"));
        bytes32 recipient = bytes32(abi.encodePacked(hex"31fcea8b515eafae980ad5afd10546f0f413c5aa7e29815699b355b30e332336"));
        bytes memory body = bytes("Hello World");
        uint256 fee = mailbox.quoteDispatch(destination, recipient, body);
        mailbox.dispatch{value: fee}(destination, recipient, body); 
    }
}
