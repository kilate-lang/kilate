require "fileutils"

def run(cmd)
  puts "#{cmd}"
  system(cmd) or abort("#{cmd} failed!")
end

# Arguments
$option_gdb = false
$option_run = false
$option_termux = false
ARGV.each do |arg|
  case arg
    when "--gdb", "-g"
      $option_gdb = true
    when "--run", "-r"
      $option_run = true
    when "--termux", "-t"
      $option_termux = true
  end
end

# Build
if $option_termux
  HOME = ENV["HOME"]
  $dest_dir = "#{HOME}/temp/rust/kilatec"
  FileUtils.mkdir_p($dest_dir)

  files = Dir.glob("**", File::FNM_DOTMATCH).reject { |f| f =~ /\A\.\.?\z/ }

  FileUtils.cp_r(files, $dest_dir, remove_destination: true)
  
  Dir.chdir($dest_dir) do
    # run("chmod -R u+x .")
    run("cargo build --bin bin")
  end
else
  run("cargo build --bin bin")
end

def run_program()
  executable = "./target/debug/bin"
  if $option_gdb
    run("gdb #{executable} --args #{executable} main.java")
  else
    run("#{executable} files/main.klt")
  end
end

# Run
if $option_termux
  Dir.chdir($dest_dir) do
    run_program() if $option_run
  end
else
  run_program() if $option_run
end