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
        address updater = address(0xb33f);
        uint256 oldValue = 0;
        uint256 newValue = 1;

        // Expect the ValueChanged event to be emitted with the new value
        vm.expectEmit(true, false, false, true);
        emit SampleContract.ValueChanged(updater, oldValue, newValue);

        // Call setValue and check if the value is updated correctly
        vm.prank(updater);
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
        address depositor = address(0xb33f);
        uint256 depositAmount = 2 ether;
        uint256 newBalance = 2 ether;
        
        // Expect the EtherReceived event to be emitted with the sender and amount
        vm.expectEmit(true, true, false, true);
        emit SampleContract.EtherReceived(depositor, depositAmount, newBalance);

        // Send 2 Ether to the contract from depositor address
        vm.deal(depositor, depositAmount);
        vm.prank(depositor);
        sampleContract.deposit{value: depositAmount}();

        // Check the contract's balance
        uint256 balance = sampleContract.getBalance();
        assertEq(balance, depositAmount, "Balance after deposit is incorrect");
    }

    function testWithdraw() public {
        address depositor = address(0xb33f);
        uint256 depositAmount = 2 ether;
        
        // Send 2 Ether to the contract from depositor address
        vm.deal(depositor, depositAmount);
        vm.prank(depositor);
        sampleContract.deposit{value: depositAmount}();

        // Check the contract's balance before withdrawal
        uint256 balanceBefore = sampleContract.getBalance();
        assertEq(balanceBefore, depositAmount, "Initial contract balance is incorrect");

        // Withdraw entire balance
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
        address depositor = address(0xb33f);
        uint256 depositAmount = 1 ether;

        // Deposit some ETH first
        vm.deal(depositor, depositAmount);
        vm.prank(depositor);
        sampleContract.deposit{value: depositAmount}();

        // Retrieve the balance using getBalance() and compare with the actual balance
        uint256 contractBalance = sampleContract.getBalance();
        assertEq(contractBalance, depositAmount, "Balance returned by getBalance() is incorrect");
    }
}
