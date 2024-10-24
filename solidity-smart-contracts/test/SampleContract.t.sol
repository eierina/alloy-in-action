// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test, console} from "forge-std/Test.sol";
import {SampleContract} from "../src/SampleContract.sol";

contract SampleContractTest is Test {
    SampleContract public sampleContract;

    function setUp() public {
        // Deploy the SampleContract with an initial value of 0
        sampleContract = new SampleContract(0);
    }

    function testSetValue() public {
        uint256 newValue = 1;

        // Expect the ValueChanged event to be emitted with the new value
        vm.expectEmit(true, false, false, true);
        emit SampleContract.ValueChanged(newValue);

        // Call setValue and check if the value is updated correctly
        sampleContract.setValue(newValue);
        uint256 currentValue = sampleContract.getValue();
        assertEq(currentValue, newValue, "Value was not set correctly");
    }

    function testGetValue() public view {
        uint256 initialValue = 0;
        uint256 currentValue = sampleContract.getValue();
        assertEq(currentValue, initialValue, "Initial value should be 0");
    }

    function testDeposit() public {
        uint256 depositAmount = 2 ether;
        address depositor = address(0xbadc0de);

        // Expect the EtherReceived event to be emitted with the sender and amount
        vm.expectEmit(true, true, false, true);
        emit SampleContract.EtherReceived(depositor, depositAmount);

        // Send 2 Ether to the contract from depositor address
        vm.deal(depositor, depositAmount); // Ensure the depositor has enough Ether
        vm.prank(depositor);
        sampleContract.deposit{value: depositAmount}();

        // Check the contract's balance
        uint256 balance = sampleContract.getBalance();
        assertEq(balance, depositAmount, "Balance after deposit is incorrect");
    }

    function testWithdraw() public {
        uint256 depositAmount = 1 ether;
        address depositor = address(0xbadc0de);

        // Send 2 Ether to the contract from depositor address
        vm.deal(depositor, 2 ether); // Ensure the depositor has enough Ether
        vm.prank(depositor);
        sampleContract.deposit{value: depositAmount}();

        // Check the contract's balance before withdrawal
        uint256 balanceBefore = sampleContract.getBalance();
        assertEq(balanceBefore, depositAmount, "Initial contract balance is incorrect");

        // Now, prank the call again so the same address (depositor) withdraws
        vm.prank(depositor);
        sampleContract.withdraw();

        // Check that the contract balance is now zero
        uint256 balanceAfter = sampleContract.getBalance();
        assertEq(balanceAfter, 0, "Contract balance after withdrawal should be 0");

        // Check that the depositor's balance has increased correctly
        uint256 depositorBalance = depositor.balance;
        assertEq(depositorBalance, 2 ether, "Withdrawal amount is incorrect");
    }

    function testRevertWithError() public {
        // Expect the SampleError to be reverted with the specific cause
        vm.expectRevert(abi.encodeWithSelector(SampleContract.SampleError.selector, "hello from revert!"));
        sampleContract.revertWithError();
    }

    function testGetBalance() public {
        uint256 depositAmount = 1 ether;

        // Deposit some ETH first
        sampleContract.deposit{value: depositAmount}();

        // Retrieve the balance using getBalance() and compare with the actual balance
        uint256 contractBalance = sampleContract.getBalance();
        assertEq(contractBalance, address(sampleContract).balance, "Balance returned by getBalance() is incorrect");
    }
}
