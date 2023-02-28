# Remove copyright year from source headers ?

From [ISSUE-2596](https://github.com/Scille/parsec-cloud/issues/2596)

Let's face it: updating copyright every year is a silly and inconvenient process (heck, I should have done this 5 months ago !)

However Google's open source documentation consider it is enough to only write the year of creation of the file:

<https://opensource.google/documentation/reference/copyright#copyright_notice>

This is confirmed by Golang project, for instance (copyright header still indicate 2016, but last modification one month old):

<https://github.com/golang/go/blob/0ab71cc065c0ce70d7df8bf498723b5a1c7a89c1/src/crypto/ed25519/ed25519.go#L1>

Facebook on it side seems to use the famous `2016-present` hack in the copyright:

<https://github.com/facebookincubator/SocketRocket/blob/6f61b5437eaf425e073309f7252c8fa5ea908311/Configurations/SocketRocket-iOS.xcconfig#L2>

MariaDB also only indicate the year of creation in their BUSL licensed code:

<https://github.com/mariadb-corporation/MaxScale/blob/1231be86fee6c935f4e09e7caa6d917fb1a22222/server/core/admin.cc#L2>

So I would say we should be fine by only putting the year of creation in the source code ;-)
