const { execFile } = require("child_process");

const runRustCli = (args, callback) => {
  const rustCliPath = "./test-genius";
  execFile(rustCliPath, args, callback);
};

if (require.main === module) {
  const args = process.argv.slice(2);
  runRustCli(args, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error executing Rust CLI: ${error}`);
      return;
    }
    if (stderr) {
      console.error(stderr);
      return;
    }
    console.log(stdout);
  });
}

module.exports = runRustCli;
