Vault of Cardboard Auxiliary Docker Images
==========================================

This directory contains sub-directories that wrap up the
instructions and build context for auxiliary OCI images used by
the Vault of Cardboard architecture / topology.

Those are:

**ux** - An nginx-derived image that brings serves up the
static HTML / JS / CSS / etc. files that make up web user
interface for Vault of Cardboard.

**api** - The backend API implementation of Vault of Cardboard.
This is the thing that does collection management, authentication,
deck and goal munging, etc.

**proxy** - A standalone squid proxy server image that can be
used in high-churn deployments, to cache data from upstream API
endpoints (like Scryfall) when that data hasn't changed.  This is
a matter of consideration and politeness to upstream operators.
