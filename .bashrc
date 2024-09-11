addkey() {
    eval "$(ssh-agent -s)"
    ssh-add --apple-use-keychain ~/.ssh/madmultimedia
}

justp() {
    # Do the ssh stuff
    addkey

    # Ask for confirmation before running git push
    read -p "Do you want to run 'git push'? (y/n) " -n 1 -r
    echo    # move to a new line
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        # Run git push if confirmed
        git push
    fi
}

alias justp=justp