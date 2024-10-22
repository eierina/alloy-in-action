// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

contract SampleContract {
    uint256 public value;

    event ValueChanged(uint256 newValue);
    event EtherReceived(address sender, uint256 amount);

    constructor(uint256 _initialValue) {
        value = _initialValue;
    }

    function setValue(uint256 _value) external {
        value = _value;
        emit ValueChanged(_value);
    }

    function getValue() external view returns (uint256 currentValue) {
        currentValue = value;
    }

    function deposit() external payable {
        emit EtherReceived(msg.sender, msg.value);
    }

    function withdraw() external {
        payable(msg.sender).transfer(address(this).balance);
    }

    function getBalance() external view returns (uint256 balance) {
        balance = address(this).balance;
    }
}