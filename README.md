# Voting Pallet 
### Why Voting
I selected the voting proyect because in my believe this kind of solutions can bring real value to companies and communities by providing a transparent mecanism of election.
Also I didn´t participate in a voting or election proyect before and seems like a good oportunity to learn a bit about it.

## General overview
##### How the voters are setted up
- Any account can be setted as a voter but is required to reserve an amount of tokens (fee) to de-sybil users.
##### How the voters get votes to participate in the proposals
- In order to get votes, a registered voter need to reserve an amount of tokens and the amount of votes that they get is the square root of this amount reserved without taking in account the register fee.
- The voters can increase their amount of votes by reserving more tokens.
##### Creation of proposals
.The proposals are created by an account with root permisions.
-It can only be one active proposal and the admin need to wait that there is no active proposal to create a new one.
#### Proposal structure
- An active proposal have different field as proposal id, the block that finalizes it, three options in wich the voters can divide their votes, a status and a hashed text.
- A finished proposal has the same parameters with one adicional field indicating wich option wins the votation.
#### Voting a proposal
- The voters that have an amount of votes greater than 0 can select how to divide their amount of token between the three options that every proposal has.
- The voters can only vote once in every active proposal even if they dont spend all of their available votes.
#### Finishing a proposal
- When the block number of the blockchain is greater than the end block of the proposal, any account can finalize the current proposal and the winner option is selected. The active proposal storage is removed and the proposal is pushed to a list of Finished Proposals.
#### Withdraw votes
- When there is no active proposals, the registered voters can call a function to free all their tokens and free the storage that keep track of them and their votes.



## Functions, transitions and storage
### Storage 
- map Voters: AccountID => votes
- map ActiveProposal: id => Proposal
- value ProposalCount
- map FinishedProposal: id => Proposal finished
- map voted proposal: accountId => proposal id

### Functions and Storage Modifications
- add_voter => Any user can call this function and modify the voters map setting his address with a voting amount of 0
-  get_votes => If a registered voter call this function reserve an amount of tokens and get the square root of the reserved amount as votes. This function impacts in the Voters map.
- set_propopsal => An account with root access can call this function when there is no active proposal and set a proposal where all the voters can vote between a max of 10 options. The active proposal storage value is modified.
- vote => the voters can call this function passing a vector of the options and amount of votes for each option. This functions modify the active proposal value by increasing the votes in the options vector.
- end_proposal => Any user can call this function when the block number is higher than the end block of the active proposal. The active proposal is deleted and stored in a map of finished proposal.
- withdraw => When there is no active proposal. The voters can withdraw their reserved tokens and clean the storage of voters.

### Steps to production
The following steps are needed to make this proyect for production:
- A method to select the current proposal.
- Game theroy models to reward and punish the good and bad actors.
- System to execute on chain calls based on the winner option.