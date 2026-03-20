class Repostat < Formula
  desc "Analyze repository complexity, track coding progress, produce AI-augmented reports"
  homepage "https://github.com/wiseyoda/repo-analysis"
  version "0.7.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/wiseyoda/repo-analysis/releases/download/v#{version}/repostat-aarch64-apple-darwin.tar.gz"
      # sha256 "UPDATE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/wiseyoda/repo-analysis/releases/download/v#{version}/repostat-x86_64-apple-darwin.tar.gz"
      # sha256 "UPDATE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    url "https://github.com/wiseyoda/repo-analysis/releases/download/v#{version}/repostat-x86_64-unknown-linux-gnu.tar.gz"
    # sha256 "UPDATE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "repostat"

    # Generate shell completions
    generate_completions_from_executable(bin/"repostat", "completions")

    # Generate man page
    man1.install Utils.safe_popen_read(bin/"repostat", "manpage").tap { |s|
      File.write("repostat.1", s)
    } => "repostat.1" rescue nil
  end

  test do
    assert_match "repostat", shell_output("#{bin}/repostat --version")
  end
end
