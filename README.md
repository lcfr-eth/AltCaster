# AltCaster - Register Alt Farcaster FID/Fnames

Problem: Warpcast the popular Farcaster client works by importing the seedphrase of the account the name/fid is registered to.  

Solution: A way to register FIDs/FNames to alternate wallets.

Alt Creator Usage:  

Create a new wallet with cast using:  
`cast wallet nm --words 24`  

Export the private key of the new wallet:  
`export PKEY=<private_key_from_cast>`  

Make sure the account is funded with some opETH then run:  
`cargo run -- --name supercoolname`

Go to your main farcaster account.  
Purchase warps.  
Gift warps to Alt account.
Sign into alt account via warpcast by entering the seed phrase into warpcast.  
Connect the account with gifted warps to warpcast and enjoy!  

Supercast maybe able to support multiple accounts

If you registered an fid but the fname was taken then you can set the fname with the following command:  
`cargo run -- --name somecoolname --fid 12345 --set-fname`

Example:
```
% cargo run -- --name qaz                                                       
   Compiling altcaster v0.1.0 (/Users/x/farcaster/altcaster)
    Finished dev [unoptimized + debuginfo] target(s) in 5.44s
     Running `target/debug/altcaster --name qaz`

Using Address: 0xa4fba9ef96ffa406b6c98a2d0962e49a4632c063
Price: 0.002613320092511532
Transaction Hash: 0x67ec10c86dea6a54d7b4cea8b69fc7f07289f5c67555d4b7f38c4c1c955e80a1
Registered FID: 301051
FNames Api Response: "{\"transfer\":{\"id\":128985,\"timestamp\":1707188217,\"username\":\"qaz\",\"owner\":\"0xa4fba9ef96ffa406b6c98a2d0962e49a4632c063\",\"from\":0,\"to\":301051,\"user_signature\":\"0x30367c7fecf0edb3bf6d90047947ae902598af10832bbe18690ef4acc89fb43c5c421b16ac9e036a0c2a0cf378f58130b7f7584f004ec2287c015874a1820e1c1b\",\"server_signature\":\"0x0350897e4498fe63670496c5ef5e301698f67b10afde588cc501fd2ad0c26c9a07a177077f1d9ebbb966bc1186c1c5d9d66e6aa15d42027fb20177ee07544efe1b\"}}"
```
