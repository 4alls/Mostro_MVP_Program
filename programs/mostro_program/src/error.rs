use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
	#[msg("Only the admin can perform this action.")]
    UnauthorizedAdmin,
	#[msg("Unauthorized access")]
	Unauthorized,
	#[msg("Percentages must sum to 100")]
	InvalidPercentage,
	#[msg("Proposal is not active")]
	ProposalNotActive,
	#[msg("Proposal has expired")]
	ProposalExpired,
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
	#[msg("Error in bonding curve calculation")]
	BondingCurveError,
	#[msg("No voting power (no tokens held)")]
	NoVotingPower,
}
