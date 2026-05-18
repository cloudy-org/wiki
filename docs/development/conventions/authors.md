# Authors

In cloudy-org we use a `AUTHORS.txt` file (also known as [AUTHORS File](https://opensource.google/documentation/reference/releasing/authors)) to define the **application author** [^1] and to **credit contributors**.

If you are the application author you place your name at the **very top**:

```txt title="AUTHORS.txt"
Goldy

# ... other contributors
```

If you're a contributor you simply append your name at the **end** of the file:

```txt title="AUTHORS.txt"
John Doe # application author

Goldy
```

The first name in the file is automatically the application author. As a contributor it is **not** required for you to list yourself in the file, it's completely up to you if you would like to be credited or not [^2].

## Optional Fields
Our `AUTHORS.txt` file also supports defining **emails** and **github profiles**.

!!! note
    Emails for contributors currently are not used anywhere and may be ignored in the future.

The formatting goes like this:

```txt
{display-name} <{email}> ({platform-tag}@{platform-handle})
```

Here's an example where `gh` is a **platform tag** for GitHub and `THEGOLDENPRO` is my GitHub **username**:

```txt title="AUTHORS.txt"
Goldy <goldy@devgoldy.xyz> (THEGOLDENPRO @ gh)
```

### Platform Tags
- `gh` = **GitHub**

## Full Example
```txt title="AUTHORS.txt"
Goldy <goldy@devgoldy.xyz> (THEGOLDENPRO @ gh)

John Doe
Sandy Cheeks
Ananas (ananasmoe @ gh)
Goldy Clone 1.0 (THEGOLDENPRO @ gh)
Goldy V2 (goldyv2 @ gh) 
```

[^1]: The **original application creator** or **another developer** who has inherited the author and maintenance role from the original creator.
[^2]: Of course you'll just always be present in git as a contributor.