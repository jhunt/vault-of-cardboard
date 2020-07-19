-- Your SQL goes here
ALTER TABLE transactions
  ADD COLUMN paid INTEGER DEFAULT NULL; -- pennies
