#!/bin/bash
curl https://api.dropbox.com/oauth2/token -d grant_type=refresh_token -d refresh_token=$DROPBOX_REFRESH_TOKEN -u $DROPBOX_APP_KEY:$DROPBOX_APP_SECRET | jq -r ".access_token" | xargs -I {} fish -C 'set -Ux DROPBOX_ACCESS_TOKEN {}'
