#!/usr/bin/perl
use Test::More;
use Test::Deep;

use LWP::UserAgent;
use MIME::Base64 qw/encode_base64/;
use JSON qw/to_json from_json/;
use Data::Dumper;

use Net::INET6Glue::INET_is_INET6;

package LWP::UserAgent;

sub patch {
	require HTTP::Request::Common;
	my($self, @parameters) = @_;
	my @suff = $self->_process_colonic_headers(\@parameters, (ref($parameters[1]) ? 2 : 1));

	# this work-around is in place as HTTP::Request::Common
	# did not implement a patch convenience method until
	# version 6.12. Once we can bump the prereq to at least
	# that version, we can use ::PATCH instead of this hack
	my $req = HTTP::Request::Common::PUT(@parameters);

	$req->method('PATCH');
	return $self->request($req, @suff);
}

package main;

sub is_uuid {
	re(qr/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/);
}

my ($UA, $URL);
sub get {
	my ($reluri, %opts) = @_;
	my $req = HTTP::Request->new(GET => "$URL$reluri", [
		Accept => 'application/json',
		($opts{as} ? (Authorization => "Basic ".encode_base64("$opts{as}:")) : ()),
	]);
	diag $req->as_string if $ENV{TRACE};
	$UA->request($req);
}

sub post {
	my ($reluri, %opts) = @_;
	my $req = HTTP::Request->new(POST => "$URL$reluri", [
			Accept => 'application/json',
			(ref($opts{payload}) ? (ContentType   => 'application/json')                   : ()),
			($opts{as}           ? (Authorization => "Basic ".encode_base64("$opts{as}:")) : ()),
		],
		(ref($opts{payload}) ? to_json($opts{payload}) : $opts{payload}),
	);
	diag $req->as_string if $ENV{TRACE};
	$UA->request($req);
}

$UA = LWP::UserAgent->new(agent => "vcb-integration-tests/1.0");
$URL = $ENV{TEST_VAULTD_URL};
ok($URL, "should have a TEST_VAULTD_URL in the environment...");

my $BULK_TOKEN = "testing-bulk-token";

######################################################################
###
###   AUTHENTICATION & SIGNUP
###
###   The following tests exercise the authentication and signup logic
###   of the Vault of Cardboard API.  Initially, authentication ought
###   to fail (since we have zero users to begin with).  However, a
###   successful signup operation should leave the user authenticated.
###

my $username = "jhunt";
my $password = "its-a-sekrit";

my $res = post("/v1/authenticate", payload => {
	username => $username,
	password => $password
});
ok($res->is_success, "authentication message should succeed")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(from_json($res->content), {
	response => {
		ok => bool(undef),
		message => 'authentication-failed',
	}
}, "bad authentication should reflect status in response payload");

my $res = post("/v1/signup", payload => {
	username => $username,
	email    => "$username\@example.com",
	password => $password
});
ok($res->is_success, "signup should succeed")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
$res = from_json($res->content);
cmp_deeply($res, {
	authenticated => {
		session  => is_uuid(),
		username => $username,
		uid      => is_uuid(),
	}
}, "successful signup should return enough information for future authentication");
my $UID = $res->{authenticated}{uid};
my $SID = $res->{authenticated}{session};

######################################################################
###
###   RETRIEVE CARD DATA
###
###   The following tests retrieve the /cards.json data created
###   by the `rescry` process, and validates a few things about
###   the structure and content of the database of all cards.
###

my $res = get("/cards.json");
ok($res->is_success, "should be able to retrieve all cards, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $cards = from_json($res->content);
cmp_deeply(
	$cards,
	superhashof({
		cards => superhashof({
			'01fc5bb3-ebd7-4ab4-8aef-2ece1e1d9b7c' => superhashof({
				name => "Lotus Vale"
			}),
		}),
		sets => superhashof({
			WTH => superhashof({
				cards => superbagof(superhashof({
					oid    => '01fc5bb3-ebd7-4ab4-8aef-2ece1e1d9b7c',
					artist => 'John Avon',
					number => '165',
				})),
			}),
		}),
	}),
	"Lotus Vale (from WTH) should exist as both an oracle card and a print card in cards.json");

######################################################################
###
###   UPDATE CARD DATA
###
###   This test attempts to upload new card data, using a modified
###   version of the on-disk card data (retrieved in the last test)
###   with key (tested-for) amounts changed.
###

$cards->{sets}{WTH}{cards} = []; # no more weatherlight!
my $res = post("/cards.json", payload => to_json($cards));
is($res->code, 401, "cards.json bulk updates must be authorized")
	or diag $res->as_string;

my $res = post("/cards.json", payload => to_json($cards), as => $BULK_TOKEN);
ok($res->is_success, "should be able to update cards.json")
	or diag $res->as_string;
ok !$res->content, 'bulk updates should return no content (http 204)'
	or diag $res->as_string;

my $res = get("/cards.json");
ok($res->is_success, "should be able to re-retrieve all cards, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $cards = from_json($res->content);
cmp_deeply(
	$cards,
	superhashof({
		cards => superhashof({
			'01fc5bb3-ebd7-4ab4-8aef-2ece1e1d9b7c' => superhashof({
				name => "Lotus Vale"
			}),
		}),
		sets => superhashof({
			WTH => superhashof({
				cards => [], # all of weatherlight just got "un" printed!
			}),
		}),
	}),
	"The WTH set is now empty (after our update)");

######################################################################
###
###   RETRIEVE PRICING DATA
###

my $res = get("/prices.json");
ok($res->is_success, "should be able to retrieve pricing data, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $prices = from_json($res->content);
is($prices->{'2e5cd12a-2a07-44a8-8eac-de00d26fe9e3'}, '19.63',
	"WTH Lotus Vale should be priced at \$19.63 (well, it shouldn't; but it WAS when the test data was retrieved...)");
ok(!exists $prices->{'01fc5bb3-ebd7-4ab4-8aef-2ece1e1d9b7c'}, "pricing should not exist for oracle cards.");

######################################################################
###
###   UPDATE PRICING DATA
###
###   This test attempts to upload new pricing data, using a modified
###   version of the on-disk pricing data (retrieved in the last test)
###   with key (tested-for) amounts changed.
###

$prices->{'2e5cd12a-2a07-44a8-8eac-de00d26fe9e3'} = '100.23';
my $res = post("/prices.json", payload =>to_json($prices));
is($res->code, 401, "cards.json bulk updates must be authorized")
	or diag $res->as_string;

my $res = post("/prices.json", payload =>to_json($prices), as => $BULK_TOKEN);
ok($res->is_success, "should be able to update prices.json")
	or diag $res->as_string;
ok !$res->content, 'bulk updates should return no content (http 204)'
	or diag $res->as_string;

my $res = get("/prices.json");
ok($res->is_success, "should be able to re-retrieve pricing data, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $prices = from_json($res->content);
is($prices->{'2e5cd12a-2a07-44a8-8eac-de00d26fe9e3'}, '100.23',
	"WTH Lotus Vale should be priced at \$100.23 (after our update)");
ok(!exists $prices->{'01fc5bb3-ebd7-4ab4-8aef-2ece1e1d9b7c'}, "pricing still should not exist for oracle cards.");

######################################################################
###
###   RETRIEVE INITIAL COLLECTION
###
###   The next test retrieves the initial collection that our new user
###   has, to verify that it is completely empty.
###

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve users default collection, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	[[], [[]]],
	"an initial collection should be empty, with zero pending operations");

######################################################################
###
###   IMPORTING THE COLLECTION
###
###   Now that we have verified that the baseline collection is empty,
###   it's time to import cards in via our first transaction.
###

my $res = get("/v1/collectors/$UID/collections/_/transactions");
ok($res->is_success, "should be able to retrieve initial (empty) transactions log, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	{
		transactions => []
	},
	"initial transactions log should be completely empty");

my $res = post("/v1/collectors/$UID/collections/_/transactions", as => $SID, payload => {
	summary => 'Initial Import',
	notes => 'This is all that\'s left of my old card collection.',
	dated => '2020-01-25',
	disposition => 'buy',
	gain => '# initial import of collection
1x MIR Enlightened Tutor
1x MIR Mystical Tutor
1x MIR Worldly Tutor',
	loss => '',
});
ok($res->is_success, "should be able to post an import transaction, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve the updated collection, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	[
		[], # initial collection, should still be empty
		[ # patches, non-empty
			[], # initial dummy patch
			set( # gain from the import transaction
				superhashof({
					quantity => 1,
					id => 'f00115bc-b551-4bf5-a121-bebb37201575' # MIR Worldly Tutor
				}),
				superhashof({
					quantity => 1,
					id => '5d98101f-e32a-4a4a-a649-faa920d111ee' # MIR Mystical Tutor
				}),
				superhashof({
					quantity => 1,
					id => 'cbac1d27-15e2-4e2f-82ab-625a16e096cb' # MIR Enlightened Tutor
				}),
			),
			[], # loss from the import transaction (i.e. none)
		],
	],
	"after posting the first (import) transaction, we should have an empty collection, and two operations");

######################################################################
###
###   UPDATING THE COLLECTION
###

my $res = post("/v1/collectors/$UID/collections/_/transactions", as => $SID, payload => {
	summary => 'More Tutoring',
	notes => 'I liked the tutors so much, I bought some more!',
	dated => '2020-01-26',
	disposition => 'buy',
	gain => '# decided to buy a bunch more
10x MIR Enlightened Tutor
20x MIR Mystical Tutor
30x MIR Worldly Tutor',
	loss => '',
});
ok($res->is_success, "should be able to post an update transaction, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve the updated collection, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	[
		[], # initial collection, should still be empty
		[   # set of patches, non-empty
			[], # initial dummy patch
			set( # gain from the import transaction
				superhashof({
					quantity => 1,
					id => 'f00115bc-b551-4bf5-a121-bebb37201575' # MIR Worldly Tutor
				}),
				superhashof({
					quantity => 1,
					id => '5d98101f-e32a-4a4a-a649-faa920d111ee' # MIR Mystical Tutor
				}),
				superhashof({
					quantity => 1,
					id => 'cbac1d27-15e2-4e2f-82ab-625a16e096cb' # MIR Enlightened Tutor
				}),
			),
			[], # loss from the import transaction (i.e. none)

			set( # gain from the second transaction
				superhashof({
					quantity => 30,
					id => 'f00115bc-b551-4bf5-a121-bebb37201575' # MIR Worldly Tutor
				}),
				superhashof({
					quantity => 20,
					id => '5d98101f-e32a-4a4a-a649-faa920d111ee' # MIR Mystical Tutor
				}),
				superhashof({
					quantity => 10,
					id => 'cbac1d27-15e2-4e2f-82ab-625a16e096cb' # MIR Enlightened Tutor
				}),
			),
			[], # loss from the second transaction (i.e. none)
		],
	],
	"after posting a second transaction, we should have an empty collection, and four operations");

######################################################################
###
###   COLLECTION RECONCILIATION
###
###   Now, we're going to trigger some server-side logic to reconcile
###   our collection down to a single [$COLLECTION], collapsing and
###   coalescing all of our gain/loss operations in the process.
###

system("cargo run --bin cardboard reconciler test/integration/fs/c/$UID/_/collection.json");
ok($? == 0, "reconciler process should run ok");

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve the reconciled collection, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	[
		set( # fully-reconciled collection
			[31, superhashof({
				pid => 'f00115bc-b551-4bf5-a121-bebb37201575', # MIR Worldly Tutor
				var => ignore,
			})],
			[21, superhashof({
				pid => '5d98101f-e32a-4a4a-a649-faa920d111ee', # MIR Mystical Tutor
				var => ignore,
			})],
			[11, superhashof({
				pid => 'cbac1d27-15e2-4e2f-82ab-625a16e096cb', # MIR Enlightened Tutor
				var => ignore,
			})],
		),
		[[]], # no patches
	],
	"after reconciling the collection, there should be zero operations, and the collection should be non-empty");

######################################################################
###
###   GOAL MANAGEMENT
###
###   These tests exercise the creation, retrieval, update, and delete
###   logic of collector goals, without actually evaluating them
###   (that's mostly a front-end thing)
###

my $res = get("/v1/collectors/$UID/goals");
ok($res->is_success, "should be able to retrieve the (empty) goal list, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	{
		goals => [],
	},
	"without creating any goals, we should get an empty goal list");

my $res = post("/v1/collectors/$UID/goals", as => $SID, payload => {
	name        => 'All Mirage commons',
	ordinal     => 0,
	target      => 'set:MIR and =common',
	goal        => 'owned',
	total       => undef,
	progress    => undef,
});
ok($res->is_success, "should be able to post a new goal, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	{
		goal => {
			id          => is_uuid(),
			collector   => $UID,
			created_at  => ignore(),
			updated_at  => ignore(),
			ordinal     => 0,
			name        => 'All Mirage commons',
			target      => 'set:MIR and =common',
			goal        => 'owned',
			total       => undef,
			progress    => undef,
		}
	},
	"creating a goal should return the goal details, as JSON");

my $res = get("/v1/collectors/$UID/goals");
ok($res->is_success, "should be able to retrieve the (now-populated) goal list, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	{
		goals => [
			{
				id          => is_uuid(),
				collector   => $UID,
				created_at  => ignore(),
				updated_at  => ignore(),
				ordinal     => 0,
				name        => 'All Mirage commons',
				target      => 'set:MIR and =common',
				goal        => 'owned',
				total       => undef,
				progress    => undef,
			},
		],
	},
	"after creating a goal, we should get it back in the main goal list");


######################################################################
###
###   DECK MANAGEMENT
###
###   These tests exercise the creation, retrieval, update, and delete
###   logic of user-owned deck lists.
###

my $res = get("/v1/collectors/$UID/decks");
ok($res->is_success, "should be able to retrieve the (empty) deck list, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	{
		decks => [],
	},
	"without creating any decks, we should get an empty deck list");

my $res = post("/v1/collectors/$UID/decks", as => $SID, payload => {
	title       => 'Mono Red Burn',
	code        => 'burrn',
	description => 'ouch ouch ouch it burns!',
	main        => "20x MIR Mountain\n".
	               "20x MIR Incinerate\n",
	side        => "15x MIR Mountain\n",
	maybe       => "15x MIR Incinerate\n",
});
ok($res->is_success, "should be able to post a new deck, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");

my $res = get("/v1/collectors/$UID/decks");
ok($res->is_success, "should be able to retrieve the (now-populated) deck list, as JSON")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(
	from_json($res->content),
	{
		decks => [
			{
				id          => is_uuid(),
				collector   => $UID,
				created_at  => ignore(),
				updated_at  => ignore(),
				lineage     => is_uuid(),
				ordinal     => 0,
				title       => 'Mono Red Burn',
				code        => 'burrn',
				description => 'ouch ouch ouch it burns!',
				main        => "20x MIR Mountain\n".
				               "20x MIR Incinerate\n",
				side        => "15x MIR Mountain\n",
				maybe       => "15x MIR Incinerate\n",
			},
		],
	},
	"after creating a deck, we should get it back in the main deck list");


######################################################################
###
###   AUTHORIZATION
###
###   These tests validate that authenticated users can only access and
###   modify their own data, and that anonymous users are mostly prohibited
###   from all sorts of things.
###
###   These tests build on the data set created previously.
###

my $res = post("/v1/signup", payload => {
	username => "not-$username",
	email    => "not+$username\@example.com",
	password => $password
});
ok($res->is_success, "signup should succeed")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
$res = from_json($res->content);
cmp_deeply($res, {
	authenticated => {
		session  => is_uuid(),
		username => "not-$username",
		uid      => is_uuid(),
	}
}, "successful signup should return enough information for future authentication");
isnt($res->{authenticated}{uid}, $UID, "should get a different UID for the not-$username user");
my $OTHER_SID = $res->{authenticated}{session};

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve $UID\'s collection, for forensics")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
my $COLLECTION = from_json($res->content);

my $res = post("/v1/collectors/$UID/collections/_/transactions", as => undef, payload => {
	dated => '2020-01-26',
	disposition => 'loss',
	gain => '# mwuahahahaha',
	loss => '# and now they are gone!
99x MIR Enlightened Tutor
99x MIR Mystical Tutor
99x MIR Worldly Tutor',
});
is($res->code, 401, "should NOT be able to post an update transaction anonymously")
	or diag $res->as_string;

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve $UID\'s collection, for forensic comparison")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(from_json($res->content), $COLLECTION, "collection should remain untouched, having foiled anonymous' plan");

my $res = post("/v1/collectors/$UID/collections/_/transactions", as => $OTHER_SID, payload => {
	dated => '2020-01-26',
	disposition => 'sell',
	gain => '# mwuahahahaha',
	loss => '# and now they are gone!
99x MIR Enlightened Tutor
99x MIR Mystical Tutor
99x MIR Worldly Tutor',
});
is($res->code, 403, "should NOT be able to post an update transaction to another collector's collection")
	or diag $res->as_string;

my $res = get("/collectors/$UID/collections/_/collection.json");
ok($res->is_success, "should be able to retrieve $UID\'s collection, for forensic comparison")
	or diag $res->as_string;
is($res->header('Content-Type'), 'application/json', "response should be JSON");
cmp_deeply(from_json($res->content), $COLLECTION, "collection should remain untouched, having foiled the other user's attempt to edit it");

######################################################################
###
###   ALL DONE
###
done_testing;
