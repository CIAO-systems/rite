#!/usr/bin/env bash
TAG_NAME=$1

git tag $TAG_NAME
git push origin $TAG_NAME