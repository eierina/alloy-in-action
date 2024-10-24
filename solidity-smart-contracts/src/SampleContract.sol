// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

/**
 * @title SampleContract - A simple contract demonstrating basic Solidity features
 * @notice This contract is a demonstration and is intended for educational purposes only.
 * @dev The code presented here is not safe for production use and may contain bugs,
 * incomplete implementations, or unsafe patterns.
 * @dev Do not use this contract in a live environment where funds or sensitive
 * information are at risk. Always conduct thorough reviews, testing, and audits
 * before deploying any smart contract on the blockchain.
 */
contract SampleContract {
    // State variable to store a single unsigned integer value
    uint256 public value;

    // Event to be emitted when the 'value' state variable is updated
    event ValueChanged(uint256 newValue);

    // Event to be emitted when Ether is received via the deposit function
    event EtherReceived(address sender, uint256 amount);

    // Custom error used to demonstrate Solidity's revert mechanism
    error SampleError(string cause);

    /// @notice Constructor to set the initial value of the contract
    /// @param _initialValue The initial value assigned to 'value'
    constructor(uint256 _initialValue) {
        value = _initialValue;
    }

    /// @notice Sets a new value for the 'value' state variable
    /// @param _value The new value to be set
    function setValue(uint256 _value) external {
        value = _value;
        emit ValueChanged(_value); // Emit event to signal the value change
    }

    /// @notice Retrieves the current value of the 'value' state variable
    /// @return currentValue The current value stored in 'value'
    function getValue() external view returns (uint256 currentValue) {
        currentValue = value;
    }

    /// @notice Accepts Ether deposits and logs the sender and amount
    function deposit() external payable {
        emit EtherReceived(msg.sender, msg.value); // Emit event to log the Ether received
    }

    /// @notice Withdraws the entire balance of the contract to the caller
    function withdraw() external {
        payable(msg.sender).transfer(address(this).balance); // Transfer balance to the caller
    }

    /// @notice Retrieves the contract's current Ether balance
    /// @return balance The current balance of the contract in wei
    function getBalance() external view returns (uint256 balance) {
        balance = address(this).balance;
    }

    /// @notice Reverts the transaction with a custom error message
    /// @dev Used to demonstrate custom error handling in Solidity
    function revertWithError() external {
        revert SampleError("hello from revert!"); // Revert with a custom error
    }
}
