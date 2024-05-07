  $ PATH=$TESTDIR/..:$PATH

Test repository setup.

  $ n=0
  $ create() {
  >   echo $1 > $1
  >   hg add $1
  >   hg commit -q -m $1 -u nobody -d "$n 0"
  >   n=$(expr $n + 1)
  > }

  $ hg init repo
  $ REPO=$(pwd)/repo

  $ cd repo
  $ for f in a b; do create $f; done
  $ hg update -q -r 0
  $ hg branch -q foo
  $ hg export -o patch 1
  $ hg import -q patch
  $ cd ..

  $ hg -R $REPO log -G --template '{node} {branch} {desc}'
  @  97b815fb8d45129120112766f8c69db8e93fbe8f foo b
  |
  | o  636e60525868096cbdc961870493510558f41d2f default b
  |/
  o  f92470d7f6966a39dfbced6a525fe81ebf5c37b9 default a
  
  $ hg -R $REPO debugdata -c 1
  a539ce0c1a22b0ecf34498f9f5ce8ea56df9ecb7
  nobody
  1 0
  b
  
  b (no-eol)
  $ hg -R $REPO debugdata -c 2
  a539ce0c1a22b0ecf34498f9f5ce8ea56df9ecb7
  nobody
  1 0 branch:foo
  b
  
  b (no-eol)

Cloning the above repository should handle the two very similar-looking
changesets properly.

  $ git -c fetch.prune=true clone -q hg::$REPO repo-git
  $ git -C repo-git cat-file -p $(git -C repo-git cinnabar hg2git 636e60525868096cbdc961870493510558f41d2f)
  tree 3683f870be446c7cc05ffaef9fa06415276e1828
  parent 8b86a58578d5270969543e287634e3a2f122a338
  author nobody <> 1 +0000
  committer nobody <> 1 +0000
  
  b (no-eol)
  $ git -C repo-git cat-file -p $(git -C repo-git cinnabar hg2git 97b815fb8d45129120112766f8c69db8e93fbe8f)
  tree 3683f870be446c7cc05ffaef9fa06415276e1828
  parent 8b86a58578d5270969543e287634e3a2f122a338
  author nobody <> 1 +0000
  committer nobody <> 1 +0000
  
  b\x00 (no-eol) (esc)

The obvious consequence is that without initial metadata, pushing this to a
mercurial repo will create a different changeset for the one in branch foo.
TODO: But we don't support creating new branches anyway, so we can't really
test it in a meaningful way.

We do have a similar problem, though, with differences in git commits that
are not handled by git-cinnabar.

  $ rm -rf $REPO

  $ n=0
  $ create() {
  >   echo $1 > $1
  >   git add $1
  >   GIT_COMMITTER_DATE="1970-01-01 0:0:$n" git -c user.name=Nobody -c user.email=nobody@nowhere commit -q -m $1 --date "1970-01-01 0:0:$n"
  >   n=$(expr $n + 1)
  > }

  $ git init -q $REPO

  $ cd $REPO
  $ for f in a b; do create $f; done
  $ cd ..
  $ git -C $REPO cat-file -p HEAD | awk '{print} /committer/{print "hidden data"}' | git -C $REPO branch foo $(git -C $REPO hash-object --stdin -w -t commit)

  $ git -C $REPO log --all --graph --oneline --no-abbrev-commit
  * 5cc73eb8dd8585b82f462fdd0df55b7c4cdf8956 b
  | * 8a1bb1e8f00cc07436f7de1cd7e5ad8b46b3306a b
  |/  
  * 0976d8403ab726134bb01bfd07f3347e74e27918 a

Equivalent to a push.
TODO: At the moment, this is not handled gracefully.

  $ git -C $REPO cinnabar bundle bundle.hd -- --all
  fatal: assertion failed: self.ids.insert(node, id).is_none()
  Run the command again with `git -c cinnabar.check=traceback <command>` to see the full traceback.
  error: git-cinnabar died of signal 6
  [134]
