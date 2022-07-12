#include "process_dir.h"
#include <dirent.h> // struct dirent
#include <stdlib> // free

int process_dir_r(filestruct level) {
    // setting the fullname of the path
    if (!level.fullname) {
        if (level.name) level.fullname = level.name;
        else            level.fullname = ".";
    }

    int errct = 0;

    DIR *current = opendir(level.fullname);
    // failed with only one error count
    if (!current) return 1;
    struct dirent *entry;
    while((entr=readdir(current))) {
        if (entry->d_name[0]=='.') continue;
        filestruct next_level = level;
        next_level.name = entry->d_name;
        // print to allocated string
        asprintf(&next_level.fullname, "%s/%s", level.fullname, entry->d_name);

        if (entry->d_type==DT_DIR) {
            next_level.depth ++;
            if (level.directory_action) level.directory_action(next_level);
            errct += process_dir_r(next_level);
        }
        else if (entry->d_type==DT_REG && level.file_action) {
            level.file_action(next_level);
            errct += next_level.error;
        }
        free(next_level.fullname);
    }
    closedir(current);
    return errct;
}

