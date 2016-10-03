// +--------------------------------------------------------------------------+
// | Copyright 2016 Matthew D. Steele <mdsteele@alum.mit.edu>                 |
// |                                                                          |
// | This file is part of System Syzygy.                                      |
// |                                                                          |
// | System Syzygy is free software: you can redistribute it and/or modify it |
// | under the terms of the GNU General Public License as published by the    |
// | Free Software Foundation, either version 3 of the License, or (at your   |
// | option) any later version.                                               |
// |                                                                          |
// | System Syzygy is distributed in the hope that it will be useful, but     |
// | WITHOUT ANY WARRANTY; without even the implied warranty of               |
// | MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU        |
// | General Public License for details.                                      |
// |                                                                          |
// | You should have received a copy of the GNU General Public License along  |
// | with System Syzygy.  If not, see <http://www.gnu.org/licenses/>.         |
// +--------------------------------------------------------------------------+

#include <Foundation/Foundation.h>

// ========================================================================= //

const char *syzygy_save_dir(char *buffer, unsigned int size) {
  // Get the path to the user's Application Support directory.
  NSArray *array =
    NSSearchPathForDirectoriesInDomains(NSApplicationSupportDirectory,
                                        NSUserDomainMask, YES);
  if ([array count] <= 0) {
    return "couldn't find app support directory";
  }
  // Append the name of our subdirectory.
  NSString *path_string =
    [[array objectAtIndex: 0] stringByAppendingString: @"/System Syzygy"];
  // Copy the directory path into the buffer.
  if (![path_string getCString: buffer
                     maxLength: size
                      encoding: NSUTF8StringEncoding]) {
    return "couldn't store path in buffer";
  }
  return "";
}

// ========================================================================= //
