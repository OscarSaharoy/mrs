# mrs

This is a small command line tool that checks your open MRs in gitlab.

Example usage:

```bash
[oscarsaharoy ~] $ mrs
You have 6 open MRs.
!2424 [need_rebase    ] ZRO-3760: address unique together fix
!2423 [not_approved   ] ZRO-3556: fix contract assignment search
!2421 [unchecked      ] ZRO-3194: lcha register flags
!2409 [not_approved   ] ZRO-3594: KYC Cleanup
!2370 [not_approved   ] Feature/zro 3622 at function frontend
!2245 [unchecked      ] ZRO-1: add extra logging and retries for pdf generation
```

To get it up and running, update the 2 lines in `src/main.rs` with your gitlab username and path to .env file containing your gitlab token. Then I set it up and linked it into my path using `cargo build; ln target/debug/mrs ~/bin/mrs` and I can just run it like `mrs` :)

