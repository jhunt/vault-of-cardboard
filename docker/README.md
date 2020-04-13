Vault of Cardboard Auxiliary Docker Images
==========================================

This directory contains sub-directories that wrap up the
instructions and build context for auxiliary OCI images used by
the Vault of Cardboard architecture / topology.

Those are:

**perimeter** - An nginx-derived image that brings serves up the
static HTML / JS / CSS / etc. files that make up web user
interface for Vault of Cardboard.

**ingester** - An alpine-based image that provides out-of-band
data ingestion and manipulation via curl + jq, from upstream
Scryfall data, into a format that Vault of Cardboard natively
understands.

**proxycache** - A standalone squid proxy server image that can be
used in high-churn deployments, to cache data from upstream API
endpoints (like Scryfall) when that data hasn't changed.  This is
a matter of consideration and politeness to upstream operators.
