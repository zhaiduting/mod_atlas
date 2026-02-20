命令 `cargo run` 执行结果如下

```
============================================================
 🌐 SCOPE: mod_atlas (Crate Root)
------------------------------------------------------------

                        child::PUBLIC => ok
                       child::PRIVATE => error
                      sibling::PUBLIC => ok
                     sibling::PRIVATE => error
         child::grandchild::PUB_CRATE => ok
           child::grandchild::PRIVATE => error

============================================================
 🌐 SCOPE: mod_atlas::child (Sub Mod)
------------------------------------------------------------

                        super::PUBLIC => ok
                       super::PRIVATE => ok
                grandchild::PUB_CRATE => ok
                grandchild::PUB_SUPER => ok
                  grandchild::PRIVATE => error
               super::sibling::PUBLIC => ok
              crate::sibling::PRIVATE => error

============================================================
 🌐 SCOPE: mod_atlas::child::grandchild (Deep Mod)
------------------------------------------------------------

                 super::super::PUBLIC => ok
                       crate::PRIVATE => ok
               crate::sibling::PUBLIC => ok
              crate::sibling::PRIVATE => error
       super::super::sibling::PRIVATE => error

============================================================
 🌐 SCOPE: mod_atlas::sibling (Private Sub-mod)
------------------------------------------------------------

                        super::PUBLIC => ok
                       super::PRIVATE => ok
                 super::child::PUBLIC => ok
     super::child::grandchild::PUBLIC => ok


```
