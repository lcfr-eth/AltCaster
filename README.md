# AltCaster - Register Alt Farcaster FID/Fnames

Problem: Warpcast the popular Farcaster client works by importing the seedphrase of the account the name/fid is registered to.  

Solution: A way to register FIDs/FNames to alternate wallets.

Alt Creator Usage:  

Create a new wallet with cast using:  
`cast wallet nm --words 24`  

Export the private key of the new wallet:  
`export PKEY=<private_key_from_cast>`  

Make sure the account is funded with some opETH then run:  
`cargo run -- --name "supercoolname"`

Go to your main farcaster account.  
Purchase warps.  
Gift warps to Alt account.
Sign into alt account via warpcast by entering the seed phrase into warpcast.  
Connect the account with gifted warps to warpcast and enjoy!  

If you registered an fid but the fname was taken then you can set the fname with the following command:  
`cargo run -- --name somecoolname --fid 12345 --set-fname`
