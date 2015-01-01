polaris
=======

implementation of the HyParView(http://www.gsd.inesc-id.pt/~jleitao/pdf/dsn07-leitao.pdf) paper in rust.

jeb's TODO list
===============
- more unit tests
- full end-to-end integration testing (and seeing if i can build tests for those)
- would like to create a Trait for message passing, to eliminate the direct opening up of connections from the library code itself
-- separate impls for 'testing', standard (what I'm doing now, opening up the connect, etc), and one for the mio project
- random improvments
-- better listening/notification when a connection closes
-- better connection handling - right now I'm just opening/closing a socket for every interaction. this is deficiency in my rust programming more than anything (maybe using mio might help relieve this)
