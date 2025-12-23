# Java Gotchas

## LSP Failing To Start

LSP was failing to start with the following error:

>   Warn  19:21:33 notify.warn Client jdtls quit with exit code 13 and signal 0. Check log for errors: /Users/peter.hessey/.local/state/nvim/lsp.log

Looking in the logs Claude suggested there were issues in the `.cache/nvim/jdtls` so I just cleared that, restarted and it seems happy now!
