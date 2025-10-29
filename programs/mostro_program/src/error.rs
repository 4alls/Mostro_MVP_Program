use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
	#[msg("Only the admin can perform this action.")]
  UnauthorizedAdmin,
	#[msg("Unauthorized access")]
	Unauthorized,
	#[msg("Percentages must sum to 100")]
	InvalidPercentage,
	#[msg("User has already voted on this proposal")]
	AlreadyVoted,
	#[msg("Insufficient tokens for operation")]
	InsufficientTokens,
	#[msg("Insufficient SOL for operation")]
	InsufficientSol,
	#[msg("Proposal was not approved")]
	ProposalNotApproved,
	#[msg("Proposal has already been executed")]
	ProposalAlreadyExecuted,
	#[msg("Invalid token amount")]
	InvalidTokenAmount,
	#[msg("Invalid SOL amount")]
	InvalidSolAmount,
	#[msg("No voting power (no tokens held)")]
	NoVotingPower,
	#[msg("Voting period has already ended.")]
  VotingEnded,
  #[msg("Arithmetic overflow occurred.")]
  Overflow,
  #[msg("Voting period is still active.")]
  VotingStillActive,
  #[msg("Invalid instruction data.")]
  InvalidInstructionData,
	#[msg("Invalid artist for this proposal.")]
  InvalidArtist,
	#[msg("Proposal has not been finalized yet.")]  
    ProposalNotFinalized,  
}
