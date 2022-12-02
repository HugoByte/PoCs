# Erc 20 
 
In this project we have created a 
Erc20 token using ink! tokens. It is the most commonly used token standard.

The Erc20 token standard defines the interface for most of the smart contracts that run on the etherum block chain.

These standars allows indiviual to deploy their cryptocurrency on top of existing platform.
Here we perform different operations like transfer of token from one account to another account and then what are the balances left in both the account after the transfer we can check we have implemented such functions where we can check these all,user balances are mapped with there account address , we have used maps for storing the values.

Intial storage consists of total_supply of tokens and balances which are present in each indiviual account. After that we can transfer tokens from one account to different account we have created a transfer function which helps in doing so , then we  can also check for the balances left in accounts after transfer.


# Flipper 

Flipper is a project which helps in when we create a smart contract , flipper generates a smart contract by default it creates a function  flip() when we run this function it will change boolean value of variable from true to false , and there is second function get() , which we get the current value of boolean variable. The lib.rs file contains two function for testing that the contract work as expected.

The flip() function is a transaction that changes the chain state and requires an account with funds to be used to execute the call. Therefore, you should select an account that has a predefined account balance, such as the alice account.



#   Incrementer 

In this project what we do is we created a function which will store the value of the variable which we give it to , first we will give it a default value to 0 , after then we can increase or decrease the value accordingly , and can store it.

we have created a function MyContract() where it will contain the value we have to give to the variable and then we have added a function that will modify the storage value accordingly. 



