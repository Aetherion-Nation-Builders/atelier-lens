# Pulling in upstream work

Upstream is [storytold/lightcraft](https://github.com/storytold/lightcraft). We merge its `main`
into ours; our changes stay a thin layer on top (`docs/brand.md`).

```bash
scripts/sync-upstream.sh            # fetch upstream, merge into a branch, run the checks
```

The script adds the `upstream` remote if it is missing, creates `sync/upstream-<date>` from
`main`, merges `upstream/main`, and leaves the checks to you (`cargo xtask ci`). Open a pull
request from that branch; `main` is protected and is only changed through pull requests.

Conflicts come from the files the brand layer touches. `NOTICE` keeps our version automatically
(`.gitattributes` marks it `merge=ours`; the script enables that merge driver). `README.md` is
our header on top of upstream's feature sections, so upstream README edits merge in below the
line; if the header conflicts, keep ours. For anything else, keep upstream's logic and re-apply
the rename.

CI here is upstream's (FreeBSD, packaging lint). Workflows turned off in this repository, on
purpose (they cost Actions minutes and build things we do not ship yet): `windows-arm64.yml`.
Turn one on from the Actions tab when it is wanted.

`release.yml` builds the macOS DMG only for now, on the standard `macos-15` runner, unsigned
(`docs/releasing.md`). Its other platform jobs are kept from upstream and skipped unless named in
the repository variable `ATELIER_RELEASE_PLATFORMS`; the DMG holds `Atelier Lens.app` on an
`Atelier Lens` volume, with the Atelier background (`packaging/macos/dmg/`). When syncing, keep
those lines of ours in `release.yml`, `packaging/macos/package.sh` and `packaging/macos/dmg/`.
