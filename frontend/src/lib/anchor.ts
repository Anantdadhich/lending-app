import { Connection, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Program, Idl } from '@project-serum/anchor';
import idl from './lending.json'; // Adjust path as needed


const PROGRAM_ID = new PublicKey('BrrQ3W1GLKS7RrL7wpP9UX39en6zkXkvZKEz8ouS8L2w'); // Use your actual program ID

export function getLendingProgram(connection: Connection, wallet: any) {
  const provider = new AnchorProvider(connection, wallet, { preflightCommitment: 'processed' });
  return new Program(idl as Idl, PROGRAM_ID, provider);
}